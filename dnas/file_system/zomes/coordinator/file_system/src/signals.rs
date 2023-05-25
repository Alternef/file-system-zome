//! This module is responsible for managing and emitting signals related to actions performed on file metadata.

use file_system_integrity::*;
use hdk::prelude::*;

/// This enum represents the possible signals that can be emitted by the Zome.
/// These signals correspond to various actions performed on file metadata.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    /// Signal that is emitted when file metadata is created.
    FileMetadataCreated {
        /// The hashed action signed by the agent.
        action: SignedActionHashed,
        /// The entry type involved in the action.
        app_entry: EntryTypes,
    },
    /// Signal that is emitted when file metadata is updated.
    FileMetadataUpdated {
        /// The hashed action signed by the agent.
        action: SignedActionHashed,
        /// The entry type involved in the action.
        app_entry: EntryTypes,
        /// The original entry type before the action was performed.
        original_app_entry: EntryTypes,
    },
    /// Signal that is emitted when file metadata is deleted.
    FileMetadataDeleted {
        /// The hashed action signed by the agent.
        action: SignedActionHashed,
        /// The original entry type before the action was performed.
        original_app_entry: EntryTypes,
    },
}

/// This function is triggered after the agent commits an action.
/// It goes through each committed action and sends the appropriate signal.
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}

/// This function is triggered after the agent commits an action.
/// It goes through each committed action and sends the appropriate signal.
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::Create(_create) => {
            let entry = get_entry_for_action(&action.hashed.hash)?;
            if let Some(EntryTypes::FileMetadata(_)) = entry {
                let signal = Signal::FileMetadataCreated {
                    action,
                    app_entry: entry.unwrap(),
                };
                emit_signal(&signal)?;
            }
            Ok(())
        }
        Action::Update(update) => {
            let entry = get_entry_for_action(&action.hashed.hash)?;
            let original_entry = get_entry_for_action(&update.original_action_address)?;
            if let Some(EntryTypes::FileMetadata(_)) = entry {
                let signal = Signal::FileMetadataUpdated {
                    action,
                    app_entry: entry.unwrap(),
                    original_app_entry: original_entry.unwrap(),
                };
                emit_signal(&signal)?;
            }
            Ok(())
        }
        Action::Delete(delete) => {
            let original_entry = get_entry_for_action(&delete.deletes_address)?;
            if let Some(EntryTypes::FileMetadata(_)) = original_entry {
                let signal = Signal::FileMetadataDeleted {
                    action,
                    original_app_entry: original_entry.unwrap(),
                };
                emit_signal(&signal)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// This helper function retrieves the entry corresponding to a given action.
/// It fetches the action's details and retrieves the corresponding entry type.
fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        })) => (zome_index, entry_index),
        _ => {
            return Ok(None);
        }
    };
    Ok(EntryTypes::deserialize_from_type(
        zome_index.clone(),
        entry_index.clone(),
        entry,
    )?)
}
