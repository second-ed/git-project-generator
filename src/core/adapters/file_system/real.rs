use crate::core::{adapters::file_system::adapter::FileSystem, errors::CoreError};
use std::{fs, path::Path};

pub struct RealFileSystem;

impl RealFileSystem {
    fn create_dirs(&mut self, path: &Path) -> Result<(), CoreError> {
        if let Some(parent) = path.parent() {
            Ok(fs::create_dir_all(parent)?)
        } else {
            Err(CoreError::FailedToCreateDirectory(path.to_path_buf()))
        }
    }
}

impl FileSystem for RealFileSystem {
    fn read_str(&self, path: &Path) -> Result<String, CoreError> {
        Ok(fs::read_to_string(path)?)
    }

    fn write_str(&mut self, path: &Path, data: String) -> Result<(), CoreError> {
        self.create_dirs(path)?;
        Ok(fs::write(path, data)?)
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn touch(&mut self, path: &Path) -> Result<(), CoreError> {
        self.create_dirs(path)?;

        let _ = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(())
    }
}
