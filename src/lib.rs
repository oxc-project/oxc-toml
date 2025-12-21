#![allow(clippy::single_match)]
//! # About
//!
//! The main purpose of this library is to format TOML documents while preserving
//! the original layout, comments, and whitespace where appropriate.
//!
//! It uses [Rowan](::rowan) for the syntax tree, and every character is preserved from the input,
//! including all comments and white space.
//!
//! # Usage
//!
//! A TOML document can be formatted directly using the [formatter::format] function:
//!
//! ```
//! use oxc_toml::formatter::{format, Options};
//!
//! const SOURCE: &str = "value=1\n[table]\nstring='some string'";
//!
//! let formatted = format(SOURCE, Options::default());
//! ```

pub mod formatter;
pub mod parser;
pub mod syntax;
pub mod util;

pub use rowan;

pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type HashSet<V> = rustc_hash::FxHashSet<V>;
