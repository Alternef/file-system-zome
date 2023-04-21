use chrono::{serde::ts_milliseconds, DateTime, Utc};
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
  #[serde(with = "ts_milliseconds")]
  pub created: DateTime<Utc>,
  #[serde(with = "ts_milliseconds")]
  pub last_modified: DateTime<Utc>,
  pub size: usize,
  // Size in bytes
  pub file_type: String,
  pub chunks_hashes: Vec<EntryHash>,
  pub path: String,
}
