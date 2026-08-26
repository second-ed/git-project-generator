use crate::core::adapters::file_system::{adapter::FileSystem, errors::FileSystemError};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct FakeFileSystem {
    pub files: HashMap<PathBuf, String>,
}

impl FakeFileSystem {
    pub fn new(files: HashMap<PathBuf, String>) -> Self {
        Self { files }
    }

    pub fn from_slice(files: &[(&str, &str)], root: &str) -> Self {
        let file_map = files
            .iter()
            .map(|(k, v)| (Path::new(root).join(k), v.to_string()))
            .collect::<HashMap<PathBuf, String>>();

        Self::new(file_map)
    }
}

impl Default for FakeFileSystem {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl FileSystem for FakeFileSystem {
    fn read_str(self, path: &Path) -> Result<String, FileSystemError> {
        if let Some(contents) = self.files.get(path) {
            Ok(contents.to_owned())
        } else {
            Err(FileSystemError::NotFound(path.to_path_buf()))
        }
    }

    fn write_str(&mut self, path: &Path, data: String) -> Result<(), FileSystemError> {
        self.files.insert(path.to_path_buf(), data.clone());
        Ok(())
    }
}
