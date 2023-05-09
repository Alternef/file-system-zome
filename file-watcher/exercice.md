# Exercise Statement: File Watcher Daemon in Rust

Objective: Develop a Rust-based file watcher daemon that monitors specific directories for changes, such as file creations, modifications, or deletions. When an event is detected, the daemon should log the event details and execute a user-defined action.

## Requirements:

1. Create a configuration file (e.g., in JSON, TOML, or YAML format) to specify the directories to be watched and the actions to be performed for each event type (file creation, modification, or deletion). The actions can be external commands or scripts.
2. Use the notify crate to monitor the specified directories for changes. The daemon should support recursive monitoring of subdirectories.
3. When a file system event occurs, log the event details, such as the event type, file path, and timestamp, using the log crate.
4. Execute the user-defined action specified in the configuration file for the corresponding event type. Ensure that you handle any errors that may arise during action execution.
5. Implement proper error handling and logging throughout the daemon. Handle situations such as invalid configurations, missing directories, or inaccessible files.
6. Daemonize the process using the daemonize crate so that it runs in the background and detaches from the terminal.
7. Include a command-line interface for starting, stopping, and checking the status of the daemon. You can use the clap crate to create the CLI.

## Stretch Goals:

1. Implement a system for sending notifications (e.g., via email or messaging services) when specific events occur.
2. Add support for monitoring file content changes and executing actions based on the content differences.
3. Implement a user-friendly configuration system that allows users to easily specify complex actions and conditions for different events.
4. Create a web dashboard to display the daemon's activity and allow users to modify the configuration and control the daemon remotely.