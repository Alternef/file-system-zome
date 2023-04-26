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
pub struct CreateFileOutput {
  pub file_metadata: Record,
  pub file_chunks: Vec<Record>,
}

#[hdk_extern]
pub fn create_file(file_input: FileInput) -> ExternResult<CreateFileOutput> {
  let chunk_size = 1024 * 1024; // 1 MB
  let file_content: Vec<u8> = file_input.content.bytes().clone();
  let num_chunks = (file_content.len() as f64 / chunk_size as f64).ceil() as usize;
  let mut chunks_hashes = Vec::new();

  for i in 0..num_chunks {
    let start = i * chunk_size;
    let end = std::cmp::min((i + 1) * chunk_size, file_content.len());
    let chunk_data = file_content[start..end].to_vec();

    let file_chunk = FileChunk(SerializedBytes::from(UnsafeBytes::from(chunk_data)));

    create_file_chunk(file_chunk.clone())?;
    let chunk_hash = hash_entry(&file_chunk)?;
    chunks_hashes.push(chunk_hash);
  }

  let author = agent_info()?.agent_initial_pubkey;
  let now = sys_time()?;

  let file_metadata = FileMetadata {
    name: file_input.name.clone(),
    author,
    path: file_input.path.clone(),
    created: now,
    last_modified: now,
    size: file_content.len(),
    file_type: file_input.file_type.clone(),
    chunks_hashes: chunks_hashes.clone(),
  };

  let metadata_record = create_file_metadata(file_metadata)?;
  let chunks_records: Vec<Record> = chunks_hashes.iter()
    .map(|chunk_hash|
      get_file_chunk(chunk_hash.clone()).unwrap())
    .collect();
  let records = CreateFileOutput {
    file_metadata: metadata_record,
    file_chunks: chunks_records,
  };

  Ok(records)
}

#[hdk_extern]
pub fn get_file_metadata(file_metadata_hash: ActionHash) -> ExternResult<Record> {
  let record = get(file_metadata_hash, GetOptions::default())?
    .ok_or(wasm_error!(WasmErrorInner::Guest("File not found".into())))?;

  Ok(record)
}

#[hdk_extern]
fn get_files_metadata_by_path(path_string: String) -> ExternResult<Vec<Record>> {
  let path = Path::from(path_string.replace("/", "."));
  get_files_metadata_recursively(path)
}
