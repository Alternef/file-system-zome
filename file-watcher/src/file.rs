use crate::get_config;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub path: String,
    pub content: String,
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(std::io::Error),
    #[error("Json error: {0}")]
    Json(serde_json::Error),
}

#[allow(dead_code)]
impl FileData {
    pub fn new(path: String, content: String) -> Self {
        Self { path, content }
    }

    pub fn write_on_fs(&self) -> Result<(), FileError> {
        let root = get_config().location;
        let path = Path::new(&root).join(&self.path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(FileError::Io)?;
            }
        }

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file: {}", err);
                return Err(FileError::Io(err));
            }
        };

        file.write_all(self.content.as_bytes())
            .map_err(FileError::Io)
    }

    pub fn save(&self) -> Result<(), FileError> {
        let file = File::open("./files.json").map_err(FileError::Io)?;
        let mut files: Vec<FileData> = serde_json::from_reader(file).map_err(FileError::Json)?;

        if let Some(index) = files.iter().position(|file| file.path == self.path) {
            files[index] = self.clone();
        } else {
            files.push(self.clone());
        }

        let file = File::create("./files.json").map_err(FileError::Io)?;
        serde_json::to_writer_pretty(file, &files).map_err(FileError::Json)
    }

    pub fn delete(&self) -> Result<(), FileError> {
        let file = File::open("./files.json").map_err(FileError::Io)?;
        let mut files: Vec<FileData> = serde_json::from_reader(file).map_err(FileError::Json)?;

        if let Some(index) = files.iter().position(|file| file.path == self.path) {
            files.remove(index);
        }

        let file = File::create("./files.json").map_err(FileError::Io)?;
        serde_json::to_writer_pretty(file, &files).map_err(FileError::Json)
    }
}

pub fn get_all_files_from_mock() -> Result<Vec<FileData>, FileError> {
    let path = Path::new("./files.json");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return Ok(vec![]);
        }
    };
    let files: Vec<FileData> = serde_json::from_reader(file).map_err(FileError::Json)?;

    for file in files.iter() {
        file.write_on_fs()?;
    }

    println!("Files synced from mock to FS: {:#?}", files);

    Ok(files)
}

pub fn get_all_files_from_fs(path: &str) -> Vec<String> {
    let mut file_paths = vec![];

    let files = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file());

    for entry in files {
        let path_str = entry.path().to_str().unwrap().to_string();
        file_paths.push(path_str);
    }

    file_paths
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_on_fs() {
        let file = FileData::new("test.txt".to_string(), "Hello World!".to_string());

        assert!(file.write_on_fs().is_ok());

        let location = get_config().location;
        let path = Path::new(&location).join("test.txt");

        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
    }
}
