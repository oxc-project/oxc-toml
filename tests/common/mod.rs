use std::fs;
use std::path::{Path, PathBuf};

pub const TOML_TEST_DIR: &str = "toml-test/tests";

/// Lists `.toml` files under `dir` that are part of TOML 1.1.0.
pub fn toml_files(dir: &str) -> Vec<PathBuf> {
    let list = Path::new(TOML_TEST_DIR).join("files-toml-1.1.0");
    let list = fs::read_to_string(&list).unwrap_or_else(|_| {
        panic!("{} not found. Please run: just clone-test-data", list.display())
    });
    let prefix = format!("{dir}/");
    list.lines()
        .filter(|l| l.starts_with(&prefix) && l.ends_with(".toml"))
        .map(|l| Path::new(TOML_TEST_DIR).join(l))
        .collect()
}
