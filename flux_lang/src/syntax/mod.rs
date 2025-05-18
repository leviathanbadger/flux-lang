//! Syntax definitions and parser.

// Include generated parser from build script
#[allow(clippy::all)]
pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/syntax/grammar.rs"));
}

pub mod ast;
pub mod lexer;
