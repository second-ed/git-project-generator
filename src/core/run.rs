use crate::core::{
    adapters::file_system::adapter::FileSystem, domain::config::Config, errors::CoreError,
};
use std::path::PathBuf;

pub fn run(adapter: &mut impl FileSystem, repo_root: PathBuf) {
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

#[cfg(test)]
mod tests {
    use crate::core::{adapters::file_system::fake::FakeFileSystem, run::run};
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
    };
    use test_case::test_case;

    fn _slice_to_files(files: &[(&str, &str)], root: &str) -> HashMap<PathBuf, String> {
        files
            .iter()
            .map(|(k, v)| (Path::new(root).join(k), v.to_string()))
            .collect::<HashMap<PathBuf, String>>()
    }

    #[test_case(
        r#"{"root_dir": ".", "template_rel_dir": "template_files", "project_name": "example-project", "files": ["src/main.rs", "src/core/mod.rs", "src/core/adapters/mod.rs", "src/core/domain/mod.rs"]}"#,
        &[
            ("config/test.json", r#"{"root_dir": ".", "template_rel_dir": "template_files", "project_name": "example-project", "files": ["src/main.rs", "src/core/mod.rs", "src/core/adapters/mod.rs", "src/core/domain/mod.rs"]}"#),
            ("example-project/src/core/adapters/mod.rs", ""),
            ("example-project/src/core/domain/mod.rs", ""),
            ("example-project/src/core/mod.rs", ""),
            ("example-project/src/main.rs", "")
        ]
    )]
    fn test_run_creates_expected_files(config_str: &str, expected_files: &[(&str, &str)]) {
        let mut file_sys =
            FakeFileSystem::from_slice(&[("config/test.json", config_str)], "some_root");

        run(&mut file_sys, Path::new("some_root").to_path_buf());

        assert_eq!(file_sys.files, _slice_to_files(expected_files, "some_root"));
    }
}
