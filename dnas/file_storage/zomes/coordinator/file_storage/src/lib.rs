use file_storage_integrity::*;
use hdk::prelude::*;

pub fn create_file_chunk(file_chunk: FileChunk) -> ExternResult<Record> {
  let file_chunk_hash = hash_entry(&file_chunk)?;

  if let None = get(file_chunk_hash.clone(), GetOptions::default())? {
    create_entry(&EntryTypes::FileChunk(file_chunk))?;
  }

  let record = get(file_chunk_hash.clone(), GetOptions::default())?
    .ok_or(wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created file chunk"))
    ))?;

  Ok(record)
}

#[hdk_extern]
pub fn create_file_metadata(file_metadata: FileMetadata) -> ExternResult<Record> {
  let action_hash = create_entry(&EntryTypes::FileMetadata(file_metadata.clone()))?;

  let record = get(action_hash.clone(), GetOptions::default())?
    .ok_or(wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created file metadata"))
    ))?;

  let file_path = file_metadata.path;
  let path = Path::from(file_path.replace("/", "."));
  let typed_path = path.typed(LinkTypes::PathToFileMetaData)?;
  typed_path.ensure()?;

  create_link(
    typed_path.path_entry_hash()?,
    action_hash.clone(),
    LinkTypes::PathToFileMetaData,
    LinkTag::new("Folder".to_string()),
  )?;

  Ok(record)
}

#[hdk_extern]
pub fn get_file_metadata(file_metadata_hash: ActionHash) -> ExternResult<Record> {
  let record = get(file_metadata_hash, GetOptions::default())?
    .ok_or(wasm_error!(WasmErrorInner::Guest("File not found".into())))?;

  Ok(record)
}

#[hdk_extern]
pub fn get_file_chunk(file_chunk_hash: EntryHash) -> ExternResult<Record> {
  let record = get(file_chunk_hash, GetOptions::default())?
    .ok_or(wasm_error!(WasmErrorInner::Guest("File not found".into())))?;

  Ok(record)
}

#[hdk_extern]
pub fn get_file_chunks(file_metadata_hash: ActionHash) -> ExternResult<Vec<Record>> {
  let record = get_file_metadata(file_metadata_hash)?;
  let file_metadata: FileMetadata = record.try_into()?;

  let mut file_chunks = Vec::new();

  if let None = file_metadata.chunks_hashes {
    return Ok(file_chunks);
  }

  for file_chunk_hash in file_metadata.chunks_hashes.unwrap() {
    let file_chunk = get_file_chunk(file_chunk_hash)?;
    file_chunks.push(file_chunk);
  }

  Ok(file_chunks)
}

#[hdk_extern]
pub fn get_all_files_metadata(_: ()) -> ExternResult<Vec<Record>> {
  let mut files = Vec::new();
  let path_entry_hash = Path::from(".").typed(LinkTypes::PathToFileMetaData)?.path_entry_hash()?;

  let paths = get_links(
    path_entry_hash,
    LinkTypes::PathToFileMetaData,
    None,
  )?;

  for path in paths {
    let file_metadata_hash = ActionHash::from(path.target);
    let file_metadata = get_file_metadata(file_metadata_hash)?;
    files.push(file_metadata);
  }

  Ok(files)
}

#[hdk_extern]
pub fn get_files_metadata_by_path(path: String) -> ExternResult<Vec<Record>> {
  let mut files = Vec::new();
  let path_entry_hash = Path::from(path.replace("/", ".")).typed(LinkTypes::PathToFileMetaData)?.path_entry_hash()?;

  let paths = get_links(
    path_entry_hash,
    LinkTypes::PathToFileMetaData,
    None,
  )?;

  for path in paths {
    let file_metadata_hash = ActionHash::from(path.target);
    let file_metadata = get_file_metadata(file_metadata_hash)?;
    files.push(file_metadata);
  }

  Ok(files)
}