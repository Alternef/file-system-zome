use hdk::prelude::*;
use file_storage_integrity::*;
use crate::get_file_metadata;

pub fn get_files_metadata_recursively(path: Path) -> ExternResult<Vec<Record>> {
  let mut files = Vec::new();

  let current_path_links = get_links(
    path.path_entry_hash()?,
    LinkTypes::PathToFileMetaData,
    None,
  )?;

  for link in current_path_links {
    let file_metadata = get_file_metadata(ActionHash::from(link.target))?;
    files.push(file_metadata);
  }

  let typed_path = path.clone().typed(LinkTypes::PathToFolderMetaData)?;
  let sub_folders_paths = typed_path.children_paths()?;

  for path in sub_folders_paths {
    let sub_files = get_files_metadata_recursively(path.path)?;
    files.extend(sub_files);
  }

  Ok(files)
}


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

pub fn create_file_metadata(file_metadata: FileMetadata) -> ExternResult<Record> {
  ensure_folder_structure(file_metadata.path.as_str())?;

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
    LinkTag::new("Folder"),
  )?;

  Ok(record)
}

pub fn create_folder_metadata(folder_metadata: FileMetadata) -> ExternResult<Record> {
  let action_hash = create_entry(&EntryTypes::FileMetadata(folder_metadata.clone()))?;

  let record = get(action_hash.clone(), GetOptions::default())?
    .ok_or(wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created folder metadata"))
    ))?;


  let folder_path = folder_metadata.path;
  let path = Path::from(folder_path.replace("/", "."));
  let typed_path = path.typed(LinkTypes::PathToFolderMetaData)?;
  typed_path.ensure()?;

  create_link(
    typed_path.path_entry_hash()?,
    action_hash.clone(),
    LinkTypes::PathToFolderMetaData,
    LinkTag::new("Folder"),
  )?;

  Ok(record)
}

pub fn ensure_folder_structure(path: &str) -> ExternResult<()> {
  let path_parts: Vec<&str> = path.split("/").collect();
  let mut current_path = String::new();

  for part in path_parts {
    if part.is_empty() { continue; }

    current_path.push_str(part);
    current_path.push('/');

    let path = Path::from(current_path.replace("/", ".")).typed(LinkTypes::PathToFolderMetaData)?;
    path.ensure()?;

    let paths = get_links(
      path.path_entry_hash()?,
      LinkTypes::PathToFolderMetaData,
      None,
    )?;

    let now = sys_time()?;

    if paths.is_empty() {
      let folder_metadata = FileMetadata {
        name: part.to_string(),
        author: agent_info()?.agent_latest_pubkey.into(),
        path: current_path.clone(),
        created: now,
        last_modified: now,
        size: 0,
        file_type: String::from("folder"),
        chunks_hashes: Vec::new(),
      };

      create_folder_metadata(folder_metadata)?;
    }
  }

  Ok(())
}

pub fn get_file_chunk(file_chunk_hash: EntryHash) -> ExternResult<Record> {
  let record = get(file_chunk_hash, GetOptions::default())?
    .ok_or(wasm_error!(WasmErrorInner::Guest("File not found".into())))?;

  Ok(record)
}

pub fn get_file_chunks(file_metadata_hash: ActionHash) -> ExternResult<Vec<Record>> {
  let record = get_file_metadata(file_metadata_hash)?;
  let file_metadata: FileMetadata = record.try_into()?;

  let mut file_chunks = Vec::new();

  if file_metadata.chunks_hashes.is_empty() {
    return Ok(file_chunks);
  }

  for file_chunk_hash in file_metadata.chunks_hashes {
    let file_chunk = get_file_chunk(file_chunk_hash)?;
    file_chunks.push(file_chunk);
  }

  Ok(file_chunks)
}