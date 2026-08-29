use crate::core::{adapters::file_system::real::RealFileSystem, run::run};
use std::path::Path;

pub mod core;

fn main() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let mut adapter = RealFileSystem;
    run(&mut adapter, repo_root);
}
