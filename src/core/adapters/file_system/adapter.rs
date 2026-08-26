use crate::core::adapters::file_system::errors::FileSystemError;
use std::path::Path;

pub trait FileSystem {
    fn read_str(&self, path: &Path) -> Result<String, FileSystemError>;
    fn write_str(&mut self, path: &Path, data: String) -> Result<(), FileSystemError>;
    fn is_dir(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;
}
