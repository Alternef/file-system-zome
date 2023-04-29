use hdk::prelude::*;
use file_storage_integrity::*;
use crate::get_file_metadata;

pub fn get_files_metadata_recursively(path: Path) -> ExternResult<Vec<Record>> {
  let mut files = Vec::new();

  let typed_path = path.clone()
    .typed(LinkTypes::PathFileSystem)?;

  let files_links = get_links(
    typed_path.path_entry_hash()?,
    LinkTypes::PathToFileMetaData,
    Some(LinkTag::new("file_metadata")),
  )?;

  for link in files_links {
    let file_metadata = get_file_metadata(ActionHash::from(link.clone().target))?;
    files.push(file_metadata);
  }

  let sub_folders_paths = typed_path.children_paths().unwrap_or_default();

  for path in sub_folders_paths {
    let sub_folder_files = get_files_metadata_recursively(path.path)?;
    files.extend(sub_folder_files)
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
  let action_hash = create_entry(&EntryTypes::FileMetadata(file_metadata.clone()))?;
  let record = get(action_hash.clone(), GetOptions::default())?
    .ok_or(wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created file metadata"))
    ))?;

  let file_path = fs_path_to_dht_path(file_metadata.path.as_str());
  let path = Path::from(file_path);
  let typed_path = path.typed(LinkTypes::PathFileSystem)?;
  typed_path.ensure()?;

  create_link(
    typed_path.path_entry_hash()?,
    action_hash.clone(),
    LinkTypes::PathToFileMetaData,
    LinkTag::new("file_metadata"),
  )?;

  Ok(record)
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


pub fn fs_path_to_dht_path(path: &str) -> String {
  let mut path = path.to_string();
  if path.starts_with("/") {
    path.remove(0);
  }
  let mut path_parts = path.split("/").collect::<Vec<&str>>();
  if path_parts[0] == "" {
    path_parts.remove(0);
  }
  path_parts.insert(0, "root");
  path_parts.join(".")
}

pub fn standardize_fs_path(path: &str) -> String {
  let mut path = path.to_string();
  if path.starts_with("/") {
    path.remove(0);
  }
  if path.ends_with("/") {
    path.pop();
  }
  path
}

#[cfg(test)]
mod files_tests {
  use super::*;

  #[test]
  fn test_fs_path_to_dht_path() {
    let path = "/test/path";
    let fs_path_to_dht_path = fs_path_to_dht_path(path);
    assert_eq!(fs_path_to_dht_path, "root.test.path");
  }

  #[test]
  fn test_standardize_fs_path() {
    let path = "/test/path/";
    let standardized_fs_path = standardize_fs_path(path);
    assert_eq!(standardized_fs_path, "test/path");
  }
}

