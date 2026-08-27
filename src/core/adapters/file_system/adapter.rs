use crate::core::errors::CoreError;
use std::path::Path;

pub trait FileSystem {
    fn read_str(&self, path: &Path) -> Result<String, CoreError>;
    fn write_str(&mut self, path: &Path, data: String) -> Result<(), CoreError>;
    fn is_dir(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;
}
