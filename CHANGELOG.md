# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.2](https://github.com/oxc-project/oxc-toml/compare/v0.14.1...v0.14.2) - 2025-12-22

### Added

- add justfile and toml-test integration tests

### Fixed

- add semantic validation for numbers and dates after logos removal

### Other

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
