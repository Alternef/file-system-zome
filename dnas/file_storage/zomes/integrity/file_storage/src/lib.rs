//! This module provides a file storage integrity zome for the Holochain ecosystem.
//! It includes entry types, link types, and validation logic for file metadata and chunks.

use hdi::prelude::*;


/// Defines link types for the integrity zome.
#[hdk_link_types]
pub enum LinkTypes {
  PathFileSystem,
  PathToFileMetaData,
  FileMetaDataUpdate,
}

/// Defines entry types for the integrity zome.
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
  FileMetadata(FileMetadata),
  FileChunk(FileChunk),
}

/// File chunk entry type.
#[hdk_entry_helper]
#[derive(Clone)]
pub struct FileChunk(pub SerializedBytes);

/// File metadata entry type.
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
