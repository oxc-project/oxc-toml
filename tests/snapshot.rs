use std::fs;
use std::path::Path;

use oxc_toml::{Options, format};

mod common;
use common::{TOML_TEST_DIR, toml_files};

#[test]
fn snapshot() {
    let valid_dir = Path::new(TOML_TEST_DIR).join("valid");

    // Collect all valid .toml files for TOML 1.1.0
    let mut files = toml_files("valid");

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
        snapshot.push_str(&format!("## {relative_path}\n\n"));

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
