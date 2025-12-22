#![allow(clippy::single_match)]
//! # About
//!
//! The main purpose of this library is to format TOML documents while preserving
//! the original layout, comments, and whitespace where appropriate.
//!
//! # Usage
//!
//! A TOML document can be formatted directly using the [formatter::format] function:
//!
//! ```
//! use oxc_toml::{format, Options};
//!
//! const SOURCE: &str = "value=1\n[table]\nstring='some string'";
//!
//! let formatted = format(SOURCE, Options::default());
//! ```

mod formatter;
mod lexer;
mod parser;
mod syntax;
mod tree;
mod util;

pub use formatter::{Options, format};
pub use parser::parse;
