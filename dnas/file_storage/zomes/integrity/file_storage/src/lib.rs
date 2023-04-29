use hdi::prelude::*;


#[hdk_link_types]
pub enum LinkTypes {
  PathFileSystem,
  PathToFileMetaData,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
  FileMetadata(FileMetadata),
  FileChunk(FileChunk),
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct FileChunk(pub SerializedBytes);

#[hdk_entry_helper]
#[derive(Clone)]
pub struct FileMetadata {
  pub name: String,
  pub author: AgentPubKey,
  pub path: String,
  pub created: Timestamp,
  pub last_modified: Timestamp,
  pub size: usize,
  pub file_type: String,
  pub chunks_hashes: Vec<EntryHash>,
}