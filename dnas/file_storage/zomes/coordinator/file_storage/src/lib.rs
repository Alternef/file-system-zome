use file_storage_integrity::*;
use hdk::prelude::*;
use files::*;

mod files;


#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
pub struct FileInput {
  pub name: String,
  pub path: String,
  pub file_type: String,
  pub content: SerializedBytes,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
pub struct FileOutput {
  pub file_metadata: Record,
  pub file_chunks: Vec<Record>,
}

#[hdk_extern]
pub fn create_file(file_input: FileInput) -> ExternResult<FileOutput> {
  let is_already_created = get_file_metadata_by_path_and_name(
    file_input.path.clone(),
    file_input.name.clone(),
  );

  if is_already_created.is_ok() {
    return Err(wasm_error!(
      WasmErrorInner::Guest(String::from("File already exists"))
    ));
  }

  let chunks_hashes = chunk_file(file_input.content.bytes().clone())?;

  let author = agent_info()?.agent_initial_pubkey;
  let now = sys_time()?;
  let fs_path = standardize_fs_path(&file_input.path);

  let file_metadata = FileMetadata {
    name: file_input.name.clone(),
    author,
    path: fs_path,
    created: now,
    last_modified: now,
    size: file_input.content.bytes().len(),
    file_type: file_input.file_type.clone(),
    chunks_hashes: chunks_hashes.clone(),
  };

  let metadata_record = create_file_metadata(file_metadata)?;
  let chunks_records: Vec<Record> = chunks_hashes.iter()
    .map(|chunk_hash|
      get_file_chunk(chunk_hash.clone()).unwrap())
    .collect();
  let records = FileOutput {
    file_metadata: metadata_record,
    file_chunks: chunks_records,
  };

  Ok(records)
}

#[hdk_extern]
pub fn get_file_chunks(file_metadata_hash: ActionHash) -> ExternResult<Vec<Record>> {
  let record_option = get_file_metadata(file_metadata_hash)?;
  if record_option.is_none() { return Ok(Vec::new()); }

  let file_metadata: FileMetadata = record_option.unwrap().try_into()?;

  let mut file_chunks = Vec::new();

  if file_metadata.chunks_hashes.is_empty() { return Ok(file_chunks); }

  for file_chunk_hash in file_metadata.chunks_hashes {
    let file_chunk = get_file_chunk(file_chunk_hash)?;
    file_chunks.push(file_chunk);
  }

  Ok(file_chunks)
}

#[hdk_extern]
pub fn get_file_metadata(original_file_metadata_hash: ActionHash) -> ExternResult<Option<Record>> {
  let links = get_links(
    original_file_metadata_hash.clone(),
    LinkTypes::FileMetaDataUpdate,
    None,
  )?;


  let latest_link = links
    .into_iter()
    .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));

  let latest_file_metadata_hash = match latest_link {
    Some(link) => ActionHash::from(link.target.clone()),
    None => original_file_metadata_hash.clone(),
  };

  get(latest_file_metadata_hash, GetOptions::default())
}

#[hdk_extern]
fn get_files_metadata_by_path_recursively(path_string: String) -> ExternResult<Vec<Record>> {
  let path_string = fs_path_to_dht_path(path_string.as_str());
  warn!("path_string: {:?}", path_string);
  let path = Path::from(path_string);

  get_files_metadata_recursively(path)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
struct UpdateFileMetadataInput {
  original_file_metadata_hash: ActionHash,
  new_content: SerializedBytes,
}

#[hdk_extern]
fn update_file(update_file_metadata_input: UpdateFileMetadataInput) -> ExternResult<FileOutput> {
  let original_file_metadata_hash = update_file_metadata_input.original_file_metadata_hash;
  let new_content = update_file_metadata_input.new_content.bytes();

  let all_update_links = get_links(
    original_file_metadata_hash.clone(),
    LinkTypes::FileMetaDataUpdate,
    None,
  )?;
  let latest_link = all_update_links
    .into_iter()
    .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));

  let previous_file_metadata_hash = match latest_link {
    Some(link) => Some(ActionHash::from(link.target.clone())),
    None => None,
  };

  let chunks_hashes = chunk_file(new_content.to_vec())?;
  let now = sys_time()?;

  let file_metadata_record = get_file_metadata(original_file_metadata_hash.clone())?
    .ok_or(wasm_error!(
      WasmErrorInner::Guest(String::from("Could not find the file metadata"))
    ))?;
  let mut file_metadata = FileMetadata::try_from(file_metadata_record.clone())?;
  let old_chunks_hashes = file_metadata.chunks_hashes.clone();

  file_metadata.last_modified = now;
  file_metadata.size = new_content.len();
  file_metadata.chunks_hashes = chunks_hashes.clone();

  for chunk_hash in old_chunks_hashes {
    let chunk_record = get_file_chunk(chunk_hash.clone())?;
    delete_entry(chunk_record.signed_action.hashed.hash)?;
  }

  let updated_metadata_record = update_file_metadata(
    original_file_metadata_hash.clone(),
    previous_file_metadata_hash.clone(),
    file_metadata,
  )?;

  let chunks_records: Vec<Record> = chunks_hashes.iter()
    .map(|chunk_hash|
      get_file_chunk(chunk_hash.clone()).unwrap())
    .collect();
  let records = FileOutput {
    file_metadata: updated_metadata_record,
    file_chunks: chunks_records,
  };

  Ok(records)
}

#[hdk_extern]
pub fn delete_file(original_file_metadata_hash: ActionHash) -> ExternResult<Vec<ActionHash>> {
  let mut delete_actions: Vec<ActionHash> = Vec::new();
  let mut update_links = get_links(
    original_file_metadata_hash.clone(),
    LinkTypes::FileMetaDataUpdate,
    None,
  )?;

  let binding = update_links.clone();
  let latest_link = binding.iter()
    .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));

  if let Some(latest_link) = latest_link {
    let latest_metadata_hash = ActionHash::from(latest_link.clone().target);

    update_links.retain(|link| link.target != latest_metadata_hash.clone().into());
    for link in update_links {
      delete_actions.push(delete_entry(ActionHash::from(link.target))?);
    }

    let file_chunks = get_file_chunks(latest_metadata_hash.clone())?;
    for file_chunk in file_chunks {
      delete_actions.push(delete_entry(file_chunk.signed_action.hashed.hash)?);
    }

    delete_actions.push(delete_entry(latest_metadata_hash)?);
  }

  delete_actions.push(delete_entry(original_file_metadata_hash)?);

  Ok(delete_actions)
}