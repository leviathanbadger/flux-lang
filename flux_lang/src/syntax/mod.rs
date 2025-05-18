//! Syntax definitions and parser.

use lalrpop_util::lalrpop_mod;

// Generated parser from grammar.lalrpop
lalrpop_mod!(pub grammar, "/syntax/grammar.rs");

pub mod ast;
pub mod lexer;
