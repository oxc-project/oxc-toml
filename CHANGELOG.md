# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.6](https://github.com/oxc-project/oxc-toml/compare/v0.14.5...v0.14.6) - 2026-07-22

### Other

- *(semgrep)* add scan workflow ([#118](https://github.com/oxc-project/oxc-toml/pull/118))
- *(deps)* update dependency dprint-json to v0.23.0 ([#113](https://github.com/oxc-project/oxc-toml/pull/113))
- *(deps)* update rust crates ([#117](https://github.com/oxc-project/oxc-toml/pull/117))
- *(deps)* update dependency rust to v1.97.1 ([#116](https://github.com/oxc-project/oxc-toml/pull/116))
- update sponsor section

## [0.14.5](https://github.com/oxc-project/oxc-toml/compare/v0.14.4...v0.14.5) - 2026-07-12

### Other

- _(deps)_ update crossbeam-epoch to v0.9.20 to fix RUSTSEC-2026-0204
- _(deps)_ update dependency rust to v1.97.0 ([#103](https://github.com/oxc-project/oxc-toml/pull/103))
- pattern-match date/time components with first_chunk ([#111](https://github.com/oxc-project/oxc-toml/pull/111))
- use str::ceil_char_boundary for UTF-8 char width ([#110](https://github.com/oxc-project/oxc-toml/pull/110))
- inline format args ([#109](https://github.com/oxc-project/oxc-toml/pull/109))
- make functions const where possible ([#108](https://github.com/oxc-project/oxc-toml/pull/108))
- use Option::is_some_and ([#107](https://github.com/oxc-project/oxc-toml/pull/107))
- use let...else for early returns ([#106](https://github.com/oxc-project/oxc-toml/pull/106))
- set rust-version to 1.95.0 ([#104](https://github.com/oxc-project/oxc-toml/pull/104))
- update snapshot for new toml-test cases ([#105](https://github.com/oxc-project/oxc-toml/pull/105))
- add Taplo fork notice ([#102](https://github.com/oxc-project/oxc-toml/pull/102))
- _(deps)_ update rust to v1.96.1 ([#101](https://github.com/oxc-project/oxc-toml/pull/101))
- _(deps)_ update rust crate ignore to v0.4.27 ([#100](https://github.com/oxc-project/oxc-toml/pull/100))
- _(deps)_ update oxc-project/security-action action to v1.0.8 ([#93](https://github.com/oxc-project/oxc-toml/pull/93))
- standardize rustfmt config
- _(deps)_ update crate-ci/typos action to v1.48.0 ([#99](https://github.com/oxc-project/oxc-toml/pull/99))
- normalize README sponsor section
- configure clippy exported api lint ([#96](https://github.com/oxc-project/oxc-toml/pull/96))
- skip new toml-test invalid duplicate-key cases (11-14) ([#95](https://github.com/oxc-project/oxc-toml/pull/95))
- add crates environment to release workflow
- add autofix workflow ([#92](https://github.com/oxc-project/oxc-toml/pull/92))
- bump cargo-shear to 1.13.1 ([#91](https://github.com/oxc-project/oxc-toml/pull/91))
- _(deps)_ update dprint plugins to v0.22.0 ([#90](https://github.com/oxc-project/oxc-toml/pull/90))
- _(deps)_ update rust crates to v1.48.0 ([#89](https://github.com/oxc-project/oxc-toml/pull/89))
- _(deps)_ update rust crates to v0.4.26 ([#88](https://github.com/oxc-project/oxc-toml/pull/88))
- _(deps)_ update typos to v1.47.2 ([#87](https://github.com/oxc-project/oxc-toml/pull/87))
- _(deps)_ update typos to v1.47.1 ([#86](https://github.com/oxc-project/oxc-toml/pull/86))
- _(deps)_ update crate-ci/typos action to v1.47.0 ([#85](https://github.com/oxc-project/oxc-toml/pull/85))
- _(deps)_ update github actions ([#84](https://github.com/oxc-project/oxc-toml/pull/84))
- _(deps)_ update dependency rust to v1.96.0 ([#83](https://github.com/oxc-project/oxc-toml/pull/83))
- _(deps)_ update crate-ci/typos action to v1.46.3 ([#82](https://github.com/oxc-project/oxc-toml/pull/82))
- _(deps)_ update dependency dprint-markdown to v0.22.1 ([#81](https://github.com/oxc-project/oxc-toml/pull/81))
- _(deps)_ update crate-ci/typos action to v1.46.2 ([#80](https://github.com/oxc-project/oxc-toml/pull/80))
- _(deps)_ update dependency dprint-markdown to v0.22.0 ([#78](https://github.com/oxc-project/oxc-toml/pull/78))
- _(deps)_ update actions/create-github-app-token action to v3.2.0 ([#77](https://github.com/oxc-project/oxc-toml/pull/77))

## [0.14.4](https://github.com/oxc-project/oxc-toml/compare/v0.14.3...v0.14.4) - 2026-05-14

### Fixed

- _(lexer)_ treat digit-led runs ending in a bare-key char as identifiers ([#73](https://github.com/oxc-project/oxc-toml/pull/73))

### Other

- use ubuntu-latest for security workflow ([#76](https://github.com/oxc-project/oxc-toml/pull/76))
- switch TextRange to Range<u32> ([#75](https://github.com/oxc-project/oxc-toml/pull/75))
- reduce memory allocations in parser and formatter ([#74](https://github.com/oxc-project/oxc-toml/pull/74))
- _(deps)_ update crate-ci/typos action to v1.46.1 ([#72](https://github.com/oxc-project/oxc-toml/pull/72))
- _(deps)_ update oxc-project/security-action action to v1.0.5 ([#70](https://github.com/oxc-project/oxc-toml/pull/70))

## [0.14.3](https://github.com/oxc-project/oxc-toml/compare/v0.14.2...v0.14.3) - 2026-05-10

### Fixed

- _(parser)_ accept uppercase E in float zero-padded check ([#69](https://github.com/oxc-project/oxc-toml/pull/69))
- fix tests with latest toml-test fixtures ([#62](https://github.com/oxc-project/oxc-toml/pull/62))

### Other

- _(deps)_ update oxc-project/security-action action to v1.0.3 ([#67](https://github.com/oxc-project/oxc-toml/pull/67))
- _(deps)_ update dependency rust to v1.95.0 ([#61](https://github.com/oxc-project/oxc-toml/pull/61))
- _(deps)_ update crate-ci/typos action to v1.46.0 ([#68](https://github.com/oxc-project/oxc-toml/pull/68))
- _(deps)_ update crate-ci/typos action to v1.45.2 ([#66](https://github.com/oxc-project/oxc-toml/pull/66))
- _(deps)_ update oxc-project/security-action action to v1.0.2 ([#65](https://github.com/oxc-project/oxc-toml/pull/65))
- make security analysis required-check friendly ([#64](https://github.com/oxc-project/oxc-toml/pull/64))
- _(deps)_ update actions/create-github-app-token action to v3.1.1 ([#58](https://github.com/oxc-project/oxc-toml/pull/58))
- add security analysis workflow ([#63](https://github.com/oxc-project/oxc-toml/pull/63))
- _(deps)_ update crate-ci/typos action to v1.45.1 ([#59](https://github.com/oxc-project/oxc-toml/pull/59))
- replace OXC_BOT_PAT with GitHub App tokens ([#60](https://github.com/oxc-project/oxc-toml/pull/60))
- _(deps)_ update rust crates ([#57](https://github.com/oxc-project/oxc-toml/pull/57))
- _(deps)_ update github-actions ([#56](https://github.com/oxc-project/oxc-toml/pull/56))
- _(deps)_ update crate-ci/typos action to v1.45.0 ([#55](https://github.com/oxc-project/oxc-toml/pull/55))
- _(deps)_ update rust crates ([#54](https://github.com/oxc-project/oxc-toml/pull/54))
- _(deps)_ update github-actions ([#53](https://github.com/oxc-project/oxc-toml/pull/53))
- _(deps)_ update dependency rust to v1.94.1 ([#52](https://github.com/oxc-project/oxc-toml/pull/52))
- _(deps)_ update github-actions ([#51](https://github.com/oxc-project/oxc-toml/pull/51))
- _(deps)_ update rust crate toml to v1.0.7 ([#50](https://github.com/oxc-project/oxc-toml/pull/50))
- _(deps)_ update dependency dprint-json to v0.21.3 ([#49](https://github.com/oxc-project/oxc-toml/pull/49))
- _(deps)_ update github-actions ([#48](https://github.com/oxc-project/oxc-toml/pull/48))
- _(deps)_ update github-actions ([#46](https://github.com/oxc-project/oxc-toml/pull/46))
- _(deps)_ update rust crate toml to v1.0.6 ([#47](https://github.com/oxc-project/oxc-toml/pull/47))
- _(deps)_ update dependency rust to v1.94.0 ([#45](https://github.com/oxc-project/oxc-toml/pull/45))
- _(deps)_ update github-actions ([#44](https://github.com/oxc-project/oxc-toml/pull/44))
- _(deps)_ update crate-ci/typos action to v1.44.0 ([#43](https://github.com/oxc-project/oxc-toml/pull/43))
- _(deps)_ update rust crate toml to v1.0.3 ([#42](https://github.com/oxc-project/oxc-toml/pull/42))
- _(deps)_ update github-actions ([#41](https://github.com/oxc-project/oxc-toml/pull/41))
- _(deps)_ update crate-ci/typos action to v1.43.5 ([#40](https://github.com/oxc-project/oxc-toml/pull/40))
- _(deps)_ update rust crate toml to v1 ([#39](https://github.com/oxc-project/oxc-toml/pull/39))
- _(deps)_ update rust crate toml to v0.9.12 ([#38](https://github.com/oxc-project/oxc-toml/pull/38))
- _(deps)_ update github-actions ([#37](https://github.com/oxc-project/oxc-toml/pull/37))
- _(deps)_ update dependency rust to v1.93.1 ([#36](https://github.com/oxc-project/oxc-toml/pull/36))
- _(deps)_ update crate-ci/typos action to v1.43.4 ([#35](https://github.com/oxc-project/oxc-toml/pull/35))
- _(deps)_ update dependency dprint-markdown to v0.21.1 ([#34](https://github.com/oxc-project/oxc-toml/pull/34))
- _(deps)_ update rust crate insta to v1.46.3 ([#33](https://github.com/oxc-project/oxc-toml/pull/33))
- _(deps)_ update github-actions ([#32](https://github.com/oxc-project/oxc-toml/pull/32))
- _(deps)_ update crate-ci/typos action to v1.43.3 ([#31](https://github.com/oxc-project/oxc-toml/pull/31))
- _(deps)_ update crate-ci/typos action to v1.43.2 ([#30](https://github.com/oxc-project/oxc-toml/pull/30))
- _(deps)_ update crate-ci/typos action to v1.43.1 ([#29](https://github.com/oxc-project/oxc-toml/pull/29))
- _(deps)_ update crate-ci/typos action to v1.43.0 ([#28](https://github.com/oxc-project/oxc-toml/pull/28))
- _(deps)_ update github-actions ([#27](https://github.com/oxc-project/oxc-toml/pull/27))
- _(deps)_ update rust crate insta to v1.46.2 ([#26](https://github.com/oxc-project/oxc-toml/pull/26))
- _(deps)_ update crate-ci/typos action to v1.42.3 ([#25](https://github.com/oxc-project/oxc-toml/pull/25))
- _(deps)_ update crate-ci/typos action to v1.42.2 ([#24](https://github.com/oxc-project/oxc-toml/pull/24))
- _(deps)_ update github-actions ([#23](https://github.com/oxc-project/oxc-toml/pull/23))
- _(deps)_ update dependency rust to v1.93.0 ([#22](https://github.com/oxc-project/oxc-toml/pull/22))
- _(deps)_ update crate-ci/typos action to v1.42.1 ([#21](https://github.com/oxc-project/oxc-toml/pull/21))
- _(deps)_ update rust crate insta to v1.46.1 ([#20](https://github.com/oxc-project/oxc-toml/pull/20))
- _(deps)_ update github-actions ([#19](https://github.com/oxc-project/oxc-toml/pull/19))
- _(deps)_ update dependency dprint-pretty_yaml to v0.6.0 ([#18](https://github.com/oxc-project/oxc-toml/pull/18))
- _(deps)_ update rust crate toml to v0.9.11 ([#17](https://github.com/oxc-project/oxc-toml/pull/17))
- _(deps)_ update github-actions ([#16](https://github.com/oxc-project/oxc-toml/pull/16))
- _(deps)_ update crate-ci/typos action to v1.42.0 ([#15](https://github.com/oxc-project/oxc-toml/pull/15))
- _(deps)_ update rust crate insta to v1.46.0 ([#14](https://github.com/oxc-project/oxc-toml/pull/14))
- _(deps)_ update taiki-e/install-action action to v2.65.12 ([#13](https://github.com/oxc-project/oxc-toml/pull/13))
- _(deps)_ update dependency dprint-json to v0.21.1 ([#12](https://github.com/oxc-project/oxc-toml/pull/12))
- _(deps)_ update crate-ci/typos action to v1.41.0 ([#11](https://github.com/oxc-project/oxc-toml/pull/11))
- _(deps)_ update crate-ci/typos action to v1.40.1 ([#10](https://github.com/oxc-project/oxc-toml/pull/10))
- _(deps)_ update github-actions ([#8](https://github.com/oxc-project/oxc-toml/pull/8))

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
- _(formatter)_ remove unused create_options macro
- disable crate unit test
- _(README)_ add sponsors info
- add deny.yml
- _(deps)_ update github-actions ([#4](https://github.com/oxc-project/oxc-toml/pull/4))
- fix cargo docs
- cargo fmt
- add CI
- Initial commit: oxc-toml formatter library
