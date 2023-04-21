use hdi::prelude::*;


#[hdk_link_types]
pub enum LinkTypes {
  PathToFileMetaData,
  FileMetaDataToChunks,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
  FileMetadata(FileMetadata),
  FileChunk(FileChunk),
}

#[hdk_entry_helper]
pub struct FileChunk(SerializedBytes);

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
  pub chunks_hashes: Option<Vec<EntryHash>>,
}
