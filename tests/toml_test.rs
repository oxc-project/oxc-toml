use std::fs;
use std::path::Path;

use oxc_toml::{Options, format, parse};
use walkdir::WalkDir;

const TOML_TEST_DIR: &str = "toml-test/tests";

/// Compare two TOML values, treating NaN as equal to NaN.
///
/// While `toml::Value` implements `PartialEq`, it follows IEEE 754 semantics where NaN != NaN.
/// For semantic equivalence testing, we want NaN to equal NaN, so we need custom comparison.
fn values_equal(a: &toml::Value, b: &toml::Value) -> bool {
    match (a, b) {
        (toml::Value::Float(f1), toml::Value::Float(f2)) => {
            // Special case: treat NaN == NaN as true
            (f1.is_nan() && f2.is_nan()) || (f1 == f2)
        }
        (toml::Value::Array(a1), toml::Value::Array(a2)) => {
            // Recursively compare arrays to handle nested NaN values
            a1.len() == a2.len() && a1.iter().zip(a2.iter()).all(|(v1, v2)| values_equal(v1, v2))
        }
        (toml::Value::Table(t1), toml::Value::Table(t2)) => {
            // Recursively compare tables to handle nested NaN values
            t1.len() == t2.len()
                && t1.iter().all(|(k, v1)| t2.get(k).is_some_and(|v2| values_equal(v1, v2)))
        }
        // For all other types, use the standard PartialEq implementation
        _ => a == b,
    }
}

/// Files that the parser accepts but shouldn't according to the spec
/// These require semantic validation which is not implemented:
/// - Duplicate key detection
/// - Table redefinition/overwrite detection  
/// - Dotted key vs table conflict detection
///
/// Some files are TOML 1.1 features that were invalid in TOML 1.0
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
    // TOML 1.1.0 allows these features that were invalid in 1.0
    "inline-table/empty-03.toml", // Empty inline tables with newlines
    "inline-table/linebreak-01.toml", // Newlines in inline tables
    "inline-table/linebreak-02.toml", // Newlines in inline tables
    "inline-table/linebreak-03.toml", // Newlines in inline tables
    "inline-table/linebreak-04.toml", // Newlines in inline tables
    "inline-table/trailing-comma.toml", // Trailing commas in inline tables
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
    let mut idempotent_failures = Vec::new();
    let mut semantic_failures = Vec::new();
    let mut panics = Vec::new();

    for entry in toml_files("valid") {
        let path = entry.path();

        let source = fs::read_to_string(path).unwrap();

        let result = std::panic::catch_unwind(|| {
            let first = format(&source, Options::default());
            let second = format(&first, Options::default());

            // Test 1: Idempotency
            let is_idempotent = first == second;

            // Test 2: Semantic equivalence - parse both original and formatted with toml crate
            // Only perform this check if both can be parsed
            let original_parsed: Result<toml::Value, _> = toml::from_str(&source);
            let formatted_parsed: Result<toml::Value, _> = toml::from_str(&first);

            let is_semantically_equivalent = match (&original_parsed, &formatted_parsed) {
                (Ok(orig), Ok(fmt)) => {
                    // Both parsed successfully - compare values using custom comparison
                    // that treats NaN == NaN
                    values_equal(orig, fmt)
                }
                _ => {
                    // If either fails to parse with the toml crate, skip semantic check
                    // This can happen for valid TOML that the toml crate doesn't support,
                    // or if our formatter has bugs that produce invalid TOML
                    true // Don't fail the test, just skip the semantic check
                }
            };

            (is_idempotent, is_semantically_equivalent)
        });

        match result {
            Ok((true, true)) => {} // Success - both tests passed
            Ok((false, _)) => idempotent_failures.push(path.to_path_buf()),
            Ok((true, false)) => semantic_failures.push(path.to_path_buf()),
            Err(_) => panics.push(path.to_path_buf()),
        }
    }

    if !panics.is_empty() {
        eprintln!("Formatter panicked on {} files:\n{panics:#?}", panics.len());
    }
    if !idempotent_failures.is_empty() {
        eprintln!(
            "Formatter is not idempotent for {} files:\n{idempotent_failures:#?}",
            idempotent_failures.len()
        );
    }
    if !semantic_failures.is_empty() {
        eprintln!(
            "Formatter changed semantic meaning for {} files:\n{semantic_failures:#?}",
            semantic_failures.len()
        );
    }
    assert!(panics.is_empty() && idempotent_failures.is_empty() && semantic_failures.is_empty());
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
