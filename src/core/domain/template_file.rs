use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, PartialOrd)]
struct TemplateFile {
    pub dst_path: PathBuf,
    pub contents: String,
}

impl TemplateFile {
    #[expect(dead_code)]
    pub fn new(dst_path: &Path, contents: String) -> Self {
        Self {
            dst_path: dst_path.to_path_buf(),
            contents,
        }
    }
}
