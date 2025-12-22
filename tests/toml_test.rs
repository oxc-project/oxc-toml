use std::fs;
use std::path::Path;

use oxc_toml::{Options, format, parse};
use walkdir::WalkDir;

const TOML_TEST_DIR: &str = "toml-test/tests";

/// Files that fail idempotent formatting (TOML 1.1.0 features not fully supported)
const SKIP_VALID: &[&str] = &["inline-table/newline-comment.toml"];

/// Files with semantic errors that the parser doesn't detect (parser only does syntactic validation)
const SKIP_INVALID: &[&str] = &[
    "array/extend-defined-aot.toml",
    "array/extending-table.toml",
    "array/tables-01.toml",
    "array/tables-02.toml",
    "control/multi-cr.toml",
    "control/rawmulti-cr.toml",
    "inline-table/duplicate-key-01.toml",
    "inline-table/duplicate-key-02.toml",
    "inline-table/duplicate-key-03.toml",
    "inline-table/duplicate-key-04.toml",
    "inline-table/overwrite-01.toml",
    "inline-table/overwrite-02.toml",
    "inline-table/overwrite-03.toml",
    "inline-table/overwrite-04.toml",
    "inline-table/overwrite-05.toml",
    "inline-table/overwrite-06.toml",
    "inline-table/overwrite-07.toml",
    "inline-table/overwrite-08.toml",
    "inline-table/overwrite-09.toml",
    "inline-table/overwrite-10.toml",
    "key/dotted-redefine-table-01.toml",
    "key/dotted-redefine-table-02.toml",
    "key/duplicate-keys-01.toml",
    "key/duplicate-keys-02.toml",
    "key/duplicate-keys-03.toml",
    "key/duplicate-keys-04.toml",
    "key/duplicate-keys-05.toml",
    "key/duplicate-keys-06.toml",
    "key/duplicate-keys-07.toml",
    "key/duplicate-keys-08.toml",
    "key/duplicate-keys-09.toml",
    "spec-1.0.0/inline-table-2-0.toml",
    "spec-1.0.0/inline-table-3-0.toml",
    "spec-1.0.0/table-9-0.toml",
    "spec-1.0.0/table-9-1.toml",
    "spec-1.1.0/common-46-0.toml",
    "spec-1.1.0/common-46-1.toml",
    "spec-1.1.0/common-49-0.toml",
    "spec-1.1.0/common-50-0.toml",
    "table/append-with-dotted-keys-01.toml",
    "table/append-with-dotted-keys-02.toml",
    "table/append-with-dotted-keys-03.toml",
    "table/append-with-dotted-keys-04.toml",
    "table/append-with-dotted-keys-05.toml",
    "table/append-with-dotted-keys-06.toml",
    "table/append-with-dotted-keys-07.toml",
    "table/array-implicit.toml",
    "table/duplicate-key-01.toml",
    "table/duplicate-key-02.toml",
    "table/duplicate-key-03.toml",
    "table/duplicate-key-04.toml",
    "table/duplicate-key-05.toml",
    "table/duplicate-key-06.toml",
    "table/duplicate-key-07.toml",
    "table/duplicate-key-08.toml",
    "table/duplicate-key-09.toml",
    "table/duplicate-key-10.toml",
    "table/overwrite-array-in-parent.toml",
    "table/overwrite-bool-with-array.toml",
    "table/overwrite-with-deep-table.toml",
    "table/redefine-01.toml",
    "table/redefine-02.toml",
    "table/redefine-03.toml",
    "table/super-twice.toml",
];

fn should_skip(path: &Path, skip_list: &[&str]) -> bool {
    let path_str = path.to_string_lossy();
    skip_list.iter().any(|skip| path_str.ends_with(skip))
}

fn toml_files(dir: &str) -> impl Iterator<Item = walkdir::DirEntry> {
    let path = Path::new(TOML_TEST_DIR).join(dir);
    assert!(
        path.exists(),
        "toml-test directory not found at {}. Please run: git submodule update --init",
        path.display()
    );
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "toml"))
}

#[test]
fn test_valid_idempotent() {
    let mut failures = Vec::new();

    for entry in toml_files("valid") {
        let path = entry.path();
        if should_skip(path, SKIP_VALID) {
            continue;
        }

        let source = fs::read_to_string(path).unwrap();

        let first = format(&source, Options::default());
        let second = format(&first, Options::default());

        if first != second {
            failures.push(path.to_path_buf());
        }
    }

    assert!(failures.is_empty(), "Formatter is not idempotent for:\n{failures:#?}");
}

#[test]
fn test_invalid_parse_failure() {
    let mut failures = Vec::new();

    for entry in toml_files("invalid") {
        let path = entry.path();
        if should_skip(path, SKIP_INVALID) {
            continue;
        }

        let Ok(source) = fs::read_to_string(path) else {
            continue; // Skip non-UTF-8 files
        };

        let result = parse(&source);

        if result.errors.is_empty() {
            failures.push(path.to_path_buf());
        }
    }

    assert!(failures.is_empty(), "Expected parse errors for:\n{failures:#?}");
}
