mod daemon;
mod file;
mod monitor;

use crate::monitor::watch_directory;
use daemon::{start_daemon, status, stop_daemon};

use crate::daemon::get_logs;
use crate::file::get_all_files_from_mock;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, process};
use toml;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    location: String,
    excludes: Option<Vec<String>>,
}

impl Config {
    fn default() -> Self {
        Self {
            location: String::from("./files"),
            excludes: Some(Vec::new()),
        }
    }
}

pub fn get_config() -> Config {
    match fs::read_to_string("./config.toml") {
        Ok(config) => toml::from_str(&config).unwrap(),
        Err(_) => Config::default(),
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "file-watcher",
    about = "A file watcher daemon",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "start", about = "Start the daemon")]
    Start,
    #[command(name = "stop", about = "Stop the daemon")]
    Stop,
    #[command(name = "status", about = "Get the daemon status")]
    Status,
    #[command(name = "restart", about = "Restart the daemon")]
    Restart,
    #[command(name = "log", about = "Get the daemon log")]
    Log,
    #[command(name = "get-files", about = "Get the files")]
    GetFiles,
}

pub fn main() {
    let config = get_config();
    let location = config.location;
    let excluded = config.excludes.unwrap_or(Vec::new());
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            start_daemon();
            watch_directory(&location, excluded.clone()).expect("Unable to watch directory");
        }
        Commands::Stop => {
            stop_daemon();
        }
        Commands::Status => {
            let result = if status() { "running" } else { "stopped" };
            println!("Daemon is {}", result);
        }
        Commands::Restart => {
            stop_daemon();
            start_daemon();
            watch_directory(&location, excluded.clone()).expect("Unable to watch directory");
        }
        Commands::Log => {
            get_logs().expect("Unable to get logs");
        }
        Commands::GetFiles => {
            get_all_files_from_mock().expect("Unable to get files");
        }
    }

    process::exit(1);
}
