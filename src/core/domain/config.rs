use crate::core::errors::CoreError;
use serde::Deserialize;

#[expect(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    root_dir: String,
    template_rel_dir: String,
    project_name: String,
    files: Vec<String>,
}

impl Config {
    pub fn new(root_dir: &str, template_rel_dir: &str, project_name: &str, files: &[&str]) -> Self {
        Self {
            root_dir: root_dir.to_string(),
            template_rel_dir: template_rel_dir.to_string(),
            project_name: project_name.to_string(),
            files: files.iter().map(|s| str::to_string(s)).collect(),
        }
    }

    pub fn from_json_str(data: String) -> Result<Self, CoreError> {
        Ok(serde_json::from_str(&data)?)
    }
}
