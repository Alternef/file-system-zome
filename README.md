# Soushi Coud

## Environment Setup

> PREREQUISITE: set up the [holochain development environment](https://developer.holochain.org/docs/install/).

Enter the nix shell by running this in the root folder of the repository:

```bash
nix-shell
npm install
```

**Run all the other instructions in this README from inside this nix-shell, otherwise they won't work**.

## Running 2 agents

```bash
npm start
```

This will create a network of 2 nodes connected to each other and their respective UIs.
It will also bring up the Holochain Playground for advanced introspection of the conductors.

## Running the backend tests

```bash
npm test
```

## Bootstrapping a network

Create a custom network of nodes connected to each other and their respective UIs with:

```bash
AGENTS=3 npm run network
```

Substitute the "3" for the number of nodes that you want to bootstrap in your network.
This will also bring up the Holochain Playground for advanced introspection of the conductors.

## Packaging

To package the web happ:

``` bash
npm run package
```

You'll have the `test-happ.webhapp` in `workdir`. This is what you should distribute so that the Holochain Launcher can
install it.
You will also have its subcomponent `test-happ.happ` in the same folder`.

## Documentation

This repository is using these tools:

- [NPM Workspaces](https://docs.npmjs.com/cli/v7/using-npm/workspaces/): npm v7's built-in monorepo capabilities.
- [hc](https://github.com/holochain/holochain/tree/develop/crates/hc): Holochain CLI to easily manage Holochain
  development instances.
- [@holochain/tryorama](https://www.npmjs.com/package/@holochain/tryorama): test framework.
- [@holochain/client](https://www.npmjs.com/package/@holochain/client): client library to connect to Holochain from the
  UI.
- [@holochain-playground/cli](https://www.npmjs.com/package/@holochain-playground/cli): introspection tooling to
  understand what's going on in the Holochain nodes.

## Architecture

### Host

#### File System DNA

Inspired by the [file-storage zome](https://github.com/holochain-open-dev/file-storage) of the [Holochain Open Dev](https://holochain-open-dev.github.io/) community.

This DNA is responsible for storing files and their metadata. It's composed of only one zome, the File_System zome.

The path of the file is managed by the path system of Holochain.

The zome allows users to upload files to the Holochain network by chunking them into smaller parts and storing those
chunks as entries.

The metadata associated with each file is also stored as an entry, and linked to the chunks via their entry hashes.

The zome provides various functions to create, read and update file metadata, retrieve file chunks, and search for files
by path recursively.

There is actually no file recovery. When a file is updated or deleted, the previous entries of the file chunks is marked as deleted and a new version is created. When getting the file chunks, the zome will return the latest version of the file chunks.

##### Entry Definitions

- `FileMetadata`: stores metadata about a file, including its name, author, path, creation date, last modification date,
  size, file type, and a list of hashes for the file chunks entries that make up the file.
- `FileChunk`: stores a chunk of a file as a serialized byte array.

##### Link Types

- `PathFileSystem`: Typed path of the file system.
- `PathToFileMetaData`: links a path to a original file_metadata entry.
- `FileMetaDataUpdate`: links file_metadata entries to their previous versions when updated.

##### Public Functions

- `create_file(file_input: FileInput) -> ExternResult<FileOutput>`:
  Creates a new file by taking a file input containing the file's name, path, type, and content. The function first
  checks
  if the file already exists and if not, chunks the file into smaller parts and creates the metadata entry for the file.
  The function then returns a record containing the file metadata entry and a list of file chunk entries.

- `get_file_chunks(file_metadata_hash: ActionHash) -> ExternResult<Vec<Record>>`:
  Retrieves a list of file chunk records associated with the specified file metadata entry hash.

- `get_file_metadata(original_file_metadata_hash: ActionHash) -> ExternResult<Option<Record>>`:
  Retrieves the latest version of a file metadata entry for the specified hash.

- `get_files_metadata_by_path_recursively(path_string: String) -> ExternResult<Vec<Record>>`:
  Retrieves all file metadata entries recursively from the specified directory path.

- `update_file(update_file_metadata_input: UpdateFileMetadataInput) -> ExternResult<FileOutput>`:
  Updates a file by creating a new version of the file metadata entry and associating it with the previous version. The
  function then returns a record containing the new file metadata entry and a list of file chunk entries.