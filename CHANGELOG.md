# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.2](https://github.com/oxc-project/oxc-toml/compare/v0.14.1...v0.14.2) - 2025-12-24

### Added

- add format_directory example using ignore crate
- add full TOML 1.1.0 support for date-times and inline tables
- add justfile and toml-test integration tests

### Fixed

- handle TOML 1.1 inline tables with newlines and comments
- add word boundary checks for keyword lexing
- remove extra spaces from empty inline tables
- don't add trailing commas to single-line arrays
- support nan/inf float values and improve inline table formatting
- add semantic validation for numbers and dates after logos removal

### Other

- fix clippy lints
- add semantic equivalence testing using toml crate
- update snapshot
- remove SKIP_VALID now that all files pass
- improve documentation for SKIP_VALID inline-table case
- change default column width to 100
- fmt
- exclude snapshot files from typos checks
- change default to disable comment alignment
- add snapshot testing for valid TOML formatting
- run fmt
- remove doc comment and disable doc tests
- remove rowan dependency
- convert remaining char operations to byte-based for performance
- use bytes instead of chars for lexer performance
- fail when toml-test directory is missing
- replace logos with manual lexer implementation
- use sort_by_cached_key to avoid repeated to_string allocations
- setup dprint
- enhance justfile with comprehensive dev workflow tasks
- add .rustfmt.toml and apply formatting
- remove itertools dependency
- remove dead code after making modules private
- only pub what's necessary
- remove unused HashMap/HashSet type aliases
- *(formatter)* remove unused create_options macro
- disable crate unit test
- *(README)* add sponsors info
- add deny.yml
- *(deps)* update github-actions ([#4](https://github.com/oxc-project/oxc-toml/pull/4))
- fix cargo docs
- cargo fmt
- add CI
- Initial commit: oxc-toml formatter library
