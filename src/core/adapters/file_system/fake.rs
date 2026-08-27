use crate::core::{adapters::file_system::adapter::FileSystem, errors::CoreError};
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
    fn read_str(&self, path: &Path) -> Result<String, CoreError> {
        if let Some(contents) = self.files.get(path) {
            Ok(contents.to_owned())
        } else {
            Err(CoreError::NotFound(path.to_path_buf()))
        }
    }

    fn write_str(&mut self, path: &Path, data: String) -> Result<(), CoreError> {
        self.files.insert(path.to_path_buf(), data.clone());
        Ok(())
    }

    fn is_dir(&self, path: &Path) -> bool {
        !self.is_file(path) && self.files.iter().any(|(file, _)| file.starts_with(path))
    }

    fn is_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn touch(&mut self, path: &Path) -> Result<(), CoreError> {
        if self.files.contains_key(path) {
            Ok(())
        } else {
            self.files.insert(path.to_path_buf(), "".to_string());
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::adapters::file_system::{adapter::FileSystem, fake::FakeFileSystem};
    use std::path::Path;
    use test_case::test_case;

    #[test_case("some_root/a/b/c.py", "x = 1" ; "given a valid python filepath, when called, then returns the string content")]
    #[test_case("some_root/d/e/f.rs", "let x = 1;" ; "given a valid rust filepath, when called, then returns the string content")]
    fn test_fake_read_write_roundtrip(input_path: &str, expected_result: &str) {
        let mut file_sys = FakeFileSystem::default();
        let path = Path::new(input_path);
        let data = expected_result.to_string();

        let write_res = file_sys.write_str(path, data);

        assert!(write_res.is_ok());
        assert_eq!(
            file_sys.read_str(path).expect("expected file to exist"),
            expected_result.to_string()
        );
    }

    #[test_case("some_root/a/b/c.py", true ; "given a valid path, when called, then returns true")]
    #[test_case("a/b/c.py", false ; "given a valid relative path without the root, when called, then returns false")]
    #[test_case("some_root/invalid_path", false ; "given an invalid path, when called, then returns false")]
    fn test_fake_is_file(input_path: &str, expected_result: bool) {
        let file_sys = FakeFileSystem::from_slice(
            &[("a/b/c.py", "x = 1"), ("d/e/f.rs", "let x = 1;")],
            "some_root",
        );
        assert_eq!(file_sys.is_file(Path::new(input_path)), expected_result);
    }

    #[test_case("some_root/a", true ; "given a valid dir path, when called, then returns true")]
    #[test_case("some_root/a/b", true ; "given a valid dir path with extra nesting, when called, then returns true")]
    #[test_case("some_root/a/b/c.py", false ; "given a valid filepath, when called, then returns false")]
    fn test_fake_is_dir(input_path: &str, expected_result: bool) {
        let file_sys = FakeFileSystem::from_slice(
            &[("a/b/c.py", "x = 1"), ("d/e/f.rs", "let x = 1;")],
            "some_root",
        );
        assert_eq!(file_sys.is_dir(Path::new(input_path)), expected_result);
    }
}
