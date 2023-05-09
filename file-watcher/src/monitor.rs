use crate::file::{get_all_files_from_fs, FileData};
use crate::get_config;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};

#[derive(Debug, PartialEq)]
enum ChangeType {
    Created,
    Modified,
    Removed,
}

struct State {
    files: HashMap<PathBuf, SystemTime>,
}

impl State {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    fn update(&mut self, path: &Path) {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                self.files.insert(path.to_owned(), modified);
            }
        }
    }

    fn remove(&mut self, path: &Path) {
        self.files.remove(path);
    }

    fn compare(&self, path: &Path, modified: SystemTime) -> ChangeType {
        match self.files.get(path) {
            Some(old_modified) => {
                if *old_modified < modified {
                    ChangeType::Modified
                } else {
                    ChangeType::Created
                }
            }
            None => ChangeType::Created,
        }
    }
}

fn process_change_event(state: &mut State, path: &Path) -> ChangeType {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            let change_type = state.compare(path, modified);
            state.update(path);
            change_type
        } else {
            ChangeType::Created
        }
    } else {
        state.remove(path);
        ChangeType::Removed
    }
}

pub fn watch_directory(path: &str, excludes: Vec<String>) -> Result<(), notify::Error> {
    let (tx, rx) = channel();
    let mut state = State::new();

    let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx).unwrap();

    for entry in get_all_files_from_fs(get_config().location.as_str()) {
        state.update(Path::new(&entry));
    }

    debouncer
        .watcher()
        .watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(events) => {
                for event in events.clone() {
                    let excluded = excludes
                        .iter()
                        .any(|exclude| event.path.to_str().unwrap().contains(exclude));
                    if excluded {
                        continue;
                    }

                    let change_type = process_change_event(&mut state, &event.path);
                    let files_from_fs = get_all_files_from_fs(path);
                    let full_path = match files_from_fs
                        .into_iter()
                        .find(|file| file.contains(event.path.to_str().unwrap()))
                    {
                        Some(full_path) => full_path,
                        None => event.path.to_str().unwrap().to_string(),
                    };

                    println!("File {:?} : {:?}", change_type, &full_path);

                    if let ChangeType::Created | ChangeType::Modified = change_type {
                        let file = FileData::new(
                            full_path.clone(),
                            fs::read_to_string(&full_path).unwrap(),
                        );
                        file.save().unwrap();
                    }
                    if let ChangeType::Removed = change_type {
                        let file = FileData::new(full_path.clone(), "".to_string());
                        file.delete().unwrap();
                    }
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}
