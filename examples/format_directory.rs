//! Format all TOML files in a directory tree.
//!
//! This example demonstrates how to use the `ignore` crate to walk a directory
//! and format all TOML files found, respecting .gitignore and other ignore files.
//!
//! Usage:
//!   cargo run --example format_directory [PATH]
//!
//! If no path is provided, it formats the current directory.

use std::env;
use std::fs;
use std::path::Path;

use ignore::WalkBuilder;
use oxc_toml::{Options, format};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("Usage: {} [PATH]", args[0]);
        println!();
        println!("Format all TOML files in a directory tree.");
        println!();
        println!("Arguments:");
        println!("  PATH    Directory to format (default: current directory)");
        println!();
        println!("This example uses the ignore crate to walk the directory,");
        println!("respecting .gitignore and other ignore files.");
        return;
    }

    let path = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let path = Path::new(path);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", path.display());
        std::process::exit(1);
    }

    println!("Formatting TOML files in: {}", path.display());

    let walker = WalkBuilder::new(path).follow_links(false).build();

    let mut formatted_count = 0;
    let mut error_count = 0;

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error walking directory: {}", err);
                error_count += 1;
                continue;
            }
        };

        let file_path = entry.path();

        // Skip directories
        if !file_path.is_file() {
            continue;
        }

        // Only process .toml files
        if file_path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }

        match fs::read_to_string(file_path) {
            Ok(source) => {
                let formatted = format(&source, Options::default());

                // Write back to file
                match fs::write(file_path, formatted) {
                    Ok(_) => {
                        println!("Formatted: {}", file_path.display());
                        formatted_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Error writing {}: {}", file_path.display(), err);
                        error_count += 1;
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading {}: {}", file_path.display(), err);
                error_count += 1;
            }
        }
    }

    println!("\nSummary:");
    println!("  Formatted: {}", formatted_count);
    println!("  Errors: {}", error_count);

    if error_count > 0 {
        std::process::exit(1);
    }
}
