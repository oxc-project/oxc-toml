use std::fs;
use std::path::Path;

use oxc_toml::{Options, format};
use walkdir::WalkDir;

const TOML_TEST_DIR: &str = "toml-test/tests";

const SKIP_VALID: &[&str] = &["inline-table/newline-comment.toml", "float/inf-and-nan.toml"];

fn should_skip(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    SKIP_VALID.iter().any(|skip| path_str.ends_with(skip))
}

#[test]
fn snapshot() {
    let valid_dir = Path::new(TOML_TEST_DIR).join("valid");

    assert!(valid_dir.exists(), "toml-test directory not found. Run: git submodule update --init");

    // Collect all valid .toml files
    let mut files: Vec<_> = WalkDir::new(&valid_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "toml"))
        .filter(|e| !should_skip(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Sort by path for consistent ordering
    files.sort();

    // Build snapshot content
    let mut snapshot = String::new();

    for (i, path) in files.iter().enumerate() {
        // Get relative path from valid/ directory
        let relative_path = path.strip_prefix(&valid_dir).unwrap().display().to_string();

        let original = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

        let formatted = format(&original, Options::default());

        // Format entry with clear comparison
        snapshot.push_str(&format!("## {}\n\n", relative_path));

        // If content is identical, show once
        if original == formatted {
            snapshot.push_str(&original);
            if !original.ends_with('\n') {
                snapshot.push('\n');
            }
        } else {
            // Show both versions with clear labels
            snapshot.push_str("Original:\n");
            snapshot.push_str(&original);
            if !original.ends_with('\n') {
                snapshot.push('\n');
            }

            snapshot.push_str("\nFormatted:\n");
            snapshot.push_str(&formatted);
            if !formatted.ends_with('\n') {
                snapshot.push('\n');
            }
        }

        // Add separator between entries (but not after last one)
        if i < files.len() - 1 {
            snapshot.push('\n');
        }
    }

    insta::assert_snapshot!(snapshot);
}
