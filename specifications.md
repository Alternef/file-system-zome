# Specifications Soushi Cloud hApp

## Introduction

Soushi Cloud hApp is an Holochain application that allows users to create and manage their own cloud storage. It is a
distributed application that can be run on any device that can run Holochain.

### Limitation
- No file chunking so the maximum file size is limited by the maximum size of a Holochain entry (1MB).

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
- `upload_file` => upload a file to the DHT
- `update_file` => update a file on the DHT
- `delete_file` => delete a file from the DHT
- `get_file` => get a file from the DHT
- `get_all_files` => get all the files from the DHT

#### Entries

##### File

```rust
pub struct File {
  pub metadata: EntryHash,
  pub content: Vec<u8>,
}
```

##### FileMetadata

```rust
pub struct FileMetadata {
  pub name: String,
  pub author: AgentPubKey,
  pub size: u64,
  pub last_modified: DateTime<Utc>,
  pub last_modified_by: AgentPubKey,
  pub path: Vec<String>,
}
```

#### Membrane Proof
