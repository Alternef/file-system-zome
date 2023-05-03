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

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    if let FlatOp::StoreEntry(store_entry) = op.flattened::<EntryTypes, LinkTypes>()? {
        match store_entry {
            OpEntry::CreateEntry { app_entry, .. } | OpEntry::UpdateEntry { app_entry, .. } => {
                match app_entry {
                    EntryTypes::FileMetadata(file_metadata) => {
                        return validate_create_file_metadata(file_metadata);
                    }
                    EntryTypes::FileChunk(_) => return Ok(ValidateCallbackResult::Valid),
                }
            }
            _ => (),
        }
    }
    Ok(ValidateCallbackResult::Valid)
}

fn validate_create_file_metadata(
    file_metadata: FileMetadata,
) -> ExternResult<ValidateCallbackResult> {
    if file_metadata.name.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            "File name cannot be empty".into(),
        ));
    }

    if has_forbidden_chars(file_metadata.path.as_str()) {
        return Ok(ValidateCallbackResult::Invalid(
            "File path cannot contain forbidden characters".into(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}

fn has_forbidden_chars(path: &str) -> bool {
    let forbidden_chars = &['<', '>', ':', '"', '|', '?', '*', '.'];
    path.chars().any(|c| forbidden_chars.contains(&c))
}
