# File Watcher Daemon

This is a simple Rust application that works as a file watcher daemon. It tracks changes to files and directories, such as creation, modification, and deletion, and can maintain a log of these changes. It can also interact with a mock file system and sync files to the actual file system.

## Structure

The project is organized into several modules, each serving a specific purpose:

1. main.rs: Entry point for the application. Contains the main function and definitions for command-line interface interactions.
2. file.rs: Defines structures and functions for handling file operations, like reading, writing, and deleting files. It also contains functions to sync files from a mock file system.
3. daemon.rs: Contains functions to control the daemon process (start, stop, and check status), as well as to retrieve logs.
4. monitor.rs: Defines functions to monitor directories for changes and handle these changes.

## Usage

The application provides a command-line interface for interacting with it. You can execute various commands:

- start: Start the daemon
- stop: Stop the daemon
- status: Get the daemon status
- restart: Restart the daemon
- log: Get the daemon log
- get-files: Get the files from the mock file system

The application reads its configuration from config.toml at its root. The configuration specifies the location of the files to watch and a list of paths to exclude from watching. If the configuration file cannot be read, the application uses a default configuration.

## Build and Run

To build the application, you need to have Rust and its package manager, Cargo, installed. Then, you can use the cargo build command in the root of the project:

```bash
cargo build
```

To run the application, use the cargo run command followed by one of the commands described in the Usage section. For example:

```bash
cargo run -- start
```

## Tests

The application includes tests in the file.rs module. To run the tests, use the cargo test command:

```bash
cargo test
```

## Dependencies

This project uses the following external crates:

- clap: For creating the command-line interface.
- serde: For serializing and deserializing data.
- daemonize: For managing the daemon process.
- notify and notify_debouncer_mini: For monitoring directories for changes.
- thiserror: For creating custom error types.
- walkdir: For recursively walking directories.
- toml: For parsing the TOML configuration file.