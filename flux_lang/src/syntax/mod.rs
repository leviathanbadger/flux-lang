//! Syntax definitions and parser.

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // Generated parser

pub mod ast;
pub mod lexer;
