#![allow(clippy::single_match)]

mod formatter;
mod lexer;
mod parser;
mod syntax;
mod tree;
mod util;

pub use formatter::{Options, format};
pub use parser::parse;
