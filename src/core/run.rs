use crate::core::{
    adapters::file_system::adapter::FileSystem, domain::config::Config, errors::CoreError,
};
use std::path::PathBuf;

pub fn run(mut adapter: impl FileSystem, repo_root: PathBuf) {
    let config = adapter
        .read_str(&repo_root.join("config/test.json"))
        .and_then(Config::from_json_str)
        .unwrap();

    let create_paths = config
        .files
        .iter()
        .map(|p| repo_root.join(format!("{}/{}", config.project_name, p)))
        .map(|path| adapter.touch(&path))
        .collect::<Vec<Result<(), CoreError>>>();
    let _ = dbg!(create_paths);
}
