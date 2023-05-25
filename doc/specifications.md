# Specifications Soushi Cloud hApp

## Introduction

Soushi Cloud hApp is an Holochain application that allows users to create and manage their own cloud storage. It is a
distributed application that can be run on any device that can run Holochain.

## Architecture

### Guest

The guest is a rust daemon that can be manage with a CLI interface.
It has the fallowing commands :

- `start` => start the daemon
- `stop` => stop the daemon
- `restart` => restart the daemon
- `status` => get the status of the daemon
- `get-files` => get the files of the daemon
- `config` => get the config of the daemon
- `logs` => get the logs of the daemon
- `version` => get the version of the daemon4
- `help` => get the help of the daemon

The daemon monitor a specific folder and upload the files to the DHT when they are created or modified. It also deletes
the files from the DHT when they are deleted and download the files from the DHT when they are requested.

### Host

The host is a Holochain DNA that is responsible for managing the files on the DHT. It has one zome named `file-storage`
that has the fallowing functions :

- `create_folder` => create a folder on the DHT
- `rename_folder` => rename a folder on the DHT
- `delete_folder` => delete a folder from the DHT
- `get_files_by_folder` => get all the files from a specific folder from the DHT
- `create_file` => create a file on the DHT
- `update_file` => update a file on the DHT
- `delete_file` => delete a file from the DHT
- `get_file` => get a file from the DHT
- `get_all_files` => get all the files from the DHT

#### Entries

##### File

```rust
pub struct FileChunk(SerializedBytes);
```

##### FileMetadata

```rust
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
```


#### Signals

##### Files

- `file_created` => signal that a file has been created
- `file_updated` => signal that a file has been updated
- `file_deleted` => signal that a file has been deleted

##### Folders

- `folder_created` => signal that a folder has been created
- `folder_renamed` => signal that a folder has been renamed
- `folder_deleted` => signal that a folder has been deleted
