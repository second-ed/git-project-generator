use crate::core::{
    adapters::file_system::{adapter::FileSystem, real::RealFileSystem},
    domain::config::Config,
};
use std::path::Path;

pub mod core;

fn main() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let adapter = RealFileSystem;
    let config = adapter
        .read_str(&repo_root.join("config/test.json"))
        .and_then(Config::from_json_str);

    let _ = dbg!(config);
}
