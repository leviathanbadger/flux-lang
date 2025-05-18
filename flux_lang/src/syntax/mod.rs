//! Syntax definitions and parser.

use lalrpop_util::lalrpop_mod;

// Include generated parser from build script
lalrpop_mod!(pub grammar, "/syntax/grammar.rs");

pub mod ast;
pub mod lexer;
