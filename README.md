# oxc-toml

A TOML v1.0.0 formatter library.

This library provides TOML formatting capabilities while preserving comments, whitespace, and the original document structure where appropriate.

## Features

- Format TOML documents according to configurable style options
- Preserve comments and meaningful whitespace
- Handle syntax errors gracefully
- Fault-tolerant parsing using [Rowan](https://github.com/rust-analyzer/rowan) syntax trees

## Usage

```rust
use oxc_toml::formatter::{format, Options};

const SOURCE: &str = "value=1\n[table]\nstring='some string'";

let formatted = format(SOURCE, Options::default());
```

## Attribution

This project is a formatter-only fork of the excellent [Taplo](https://github.com/tamasfe/taplo) project, originally created by [Ferenc Tamás](https://github.com/tamasfe).

### What Changed

This fork strips away all non-formatter components from Taplo, including:
- CLI tool
- Language Server Protocol (LSP) implementation
- WebAssembly bindings
- DOM (Document Object Model) for TOML manipulation
- JavaScript/TypeScript packages
- Editor integrations

The result is a focused, lightweight library that does one thing well: format TOML documents.

### Original Taplo Project

Taplo is a comprehensive TOML toolkit that provides:
- TOML v1.0.0 parser
- Formatter (the foundation of this fork)
- Language server for IDE integration
- CLI tool for formatting and validation
- WebAssembly bindings for browser/Node.js
- Schema validation

If you need these features, please use the original [Taplo project](https://github.com/tamasfe/taplo).

### License

This project maintains the original MIT License from Taplo. See [LICENSE](LICENSE) for details.
