//! Syntax definitions and parser.

// Include generated parser from build script
#[allow(clippy::all)]
pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/syntax/grammar.rs"));
}

pub mod ast;
pub mod lexer;

/// Error returned when parsing source fails.
#[derive(Debug)]
pub struct ParseError {
    /// Zero-based line of the error.
    pub line: usize,
    /// Zero-based column of the error.
    pub column: usize,
    /// Human readable error message.
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}
