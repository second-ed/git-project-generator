use crate::core::adapters::file_system::{adapter::FileSystem, errors::FileSystemError};
use std::fs;

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_str(self, path: &std::path::Path) -> Result<String, FileSystemError> {
        Ok(fs::read_to_string(path)?)
    }

    fn write_str(&mut self, path: &std::path::Path, data: String) -> Result<(), FileSystemError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
            Ok(fs::write(path, data)?)
        } else {
            Err(FileSystemError::FailedToCreateDirectory(path.to_path_buf()))
        }
    }
}
