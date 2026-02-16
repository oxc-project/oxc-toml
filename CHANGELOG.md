# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.3](https://github.com/oxc-project/oxc-toml/compare/v0.14.2...v0.14.3) - 2026-02-16

### Other

- *(deps)* update crate-ci/typos action to v1.43.5 ([#40](https://github.com/oxc-project/oxc-toml/pull/40))
- *(deps)* update rust crate toml to v1 ([#39](https://github.com/oxc-project/oxc-toml/pull/39))
- *(deps)* update rust crate toml to v0.9.12 ([#38](https://github.com/oxc-project/oxc-toml/pull/38))
- *(deps)* update github-actions ([#37](https://github.com/oxc-project/oxc-toml/pull/37))
- *(deps)* update dependency rust to v1.93.1 ([#36](https://github.com/oxc-project/oxc-toml/pull/36))
- *(deps)* update crate-ci/typos action to v1.43.4 ([#35](https://github.com/oxc-project/oxc-toml/pull/35))
- *(deps)* update dependency dprint-markdown to v0.21.1 ([#34](https://github.com/oxc-project/oxc-toml/pull/34))
- *(deps)* update rust crate insta to v1.46.3 ([#33](https://github.com/oxc-project/oxc-toml/pull/33))
- *(deps)* update github-actions ([#32](https://github.com/oxc-project/oxc-toml/pull/32))
- *(deps)* update crate-ci/typos action to v1.43.3 ([#31](https://github.com/oxc-project/oxc-toml/pull/31))
- *(deps)* update crate-ci/typos action to v1.43.2 ([#30](https://github.com/oxc-project/oxc-toml/pull/30))
- *(deps)* update crate-ci/typos action to v1.43.1 ([#29](https://github.com/oxc-project/oxc-toml/pull/29))
- *(deps)* update crate-ci/typos action to v1.43.0 ([#28](https://github.com/oxc-project/oxc-toml/pull/28))
- *(deps)* update github-actions ([#27](https://github.com/oxc-project/oxc-toml/pull/27))
- *(deps)* update rust crate insta to v1.46.2 ([#26](https://github.com/oxc-project/oxc-toml/pull/26))
- *(deps)* update crate-ci/typos action to v1.42.3 ([#25](https://github.com/oxc-project/oxc-toml/pull/25))
- *(deps)* update crate-ci/typos action to v1.42.2 ([#24](https://github.com/oxc-project/oxc-toml/pull/24))
- *(deps)* update github-actions ([#23](https://github.com/oxc-project/oxc-toml/pull/23))
- *(deps)* update dependency rust to v1.93.0 ([#22](https://github.com/oxc-project/oxc-toml/pull/22))
- *(deps)* update crate-ci/typos action to v1.42.1 ([#21](https://github.com/oxc-project/oxc-toml/pull/21))
- *(deps)* update rust crate insta to v1.46.1 ([#20](https://github.com/oxc-project/oxc-toml/pull/20))
- *(deps)* update github-actions ([#19](https://github.com/oxc-project/oxc-toml/pull/19))
- *(deps)* update dependency dprint-pretty_yaml to v0.6.0 ([#18](https://github.com/oxc-project/oxc-toml/pull/18))
- *(deps)* update rust crate toml to v0.9.11 ([#17](https://github.com/oxc-project/oxc-toml/pull/17))
- *(deps)* update github-actions ([#16](https://github.com/oxc-project/oxc-toml/pull/16))
- *(deps)* update crate-ci/typos action to v1.42.0 ([#15](https://github.com/oxc-project/oxc-toml/pull/15))
- *(deps)* update rust crate insta to v1.46.0 ([#14](https://github.com/oxc-project/oxc-toml/pull/14))
- *(deps)* update taiki-e/install-action action to v2.65.12 ([#13](https://github.com/oxc-project/oxc-toml/pull/13))
- *(deps)* update dependency dprint-json to v0.21.1 ([#12](https://github.com/oxc-project/oxc-toml/pull/12))
- *(deps)* update crate-ci/typos action to v1.41.0 ([#11](https://github.com/oxc-project/oxc-toml/pull/11))
- *(deps)* update crate-ci/typos action to v1.40.1 ([#10](https://github.com/oxc-project/oxc-toml/pull/10))
- *(deps)* update github-actions ([#8](https://github.com/oxc-project/oxc-toml/pull/8))

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
