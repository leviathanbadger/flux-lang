//! Core compiler library for FluxLang.

pub mod codegen;
pub mod ir;
pub mod macros;
pub mod plugins;
pub mod semantic;
pub mod syntax;

use anyhow::{anyhow, Result};
use lalrpop_util::ParseError as LalrpopError;

/// Stub compile entry point.
pub fn compile(source: &str) -> Result<()> {
    compile_with_backend(source, codegen::Backend::Llvm)
}

/// Parse FluxLang source into an AST.
pub fn parse_program(source: &str) -> Result<syntax::ast::Program> {
    syntax::grammar::ProgramParser::new()
        .parse(source)
        .map_err(|e: LalrpopError<usize, _, _>| {
            use lalrpop_util::ParseError::*;
            let (offset, message) = match e {
                InvalidToken { location } => (location, "invalid token".to_string()),
                UnrecognizedEof { location, expected } => (
                    location,
                    format!("unexpected end of input, expected {}", expected.join(", ")),
                ),
                UnrecognizedToken {
                    token: (loc, _, _),
                    expected,
                } => (
                    loc,
                    format!("unexpected token, expected {}", expected.join(", ")),
                ),
                ExtraToken { token: (loc, _, _) } => (loc, "extra token".to_string()),
                User { error } => (0, error.to_string()),
            };
            let (line, column) = syntax::util::offset_to_line_col(source, offset);
            anyhow!(syntax::ParseError {
                line,
                column,
                message
            })
        })
}

/// Compile FluxLang source using the specified backend.
pub fn compile_with_backend(source: &str, backend: codegen::Backend) -> Result<()> {
    // Parse source into AST
    let mut ast = parse_program(source)?;

    // Expand macros
    expand_macros(&mut ast)?;

    // Register and run development plugins
    run_plugins(&mut ast)?;

    // Type check
    type_check(&ast)?;

    // Lower to IR and optimize
    let ir = lower_and_optimize(&ast)?;

    // Emit code (placeholder)
    emit_code(&ir, backend)?;
    Ok(())
}

fn expand_macros(program: &mut syntax::ast::Program) -> Result<()> {
    macros::expand(program);
    Ok(())
}

fn run_plugins(program: &mut syntax::ast::Program) -> Result<()> {
    if std::env::var_os("FLUX_SKIP_DEFAULT_PLUGINS").is_none() {
        plugins::register_default_plugins();
    }
    plugins::run_all(program);
    Ok(())
}

fn type_check(program: &syntax::ast::Program) -> Result<()> {
    semantic::check(program)?;
    Ok(())
}

fn lower_and_optimize(program: &syntax::ast::Program) -> Result<ir::IrModule> {
    let mut ir = ir::lower(program);
    ir::run_passes(&mut ir);
    Ok(ir)
}

fn emit_code(ir: &ir::IrModule, backend: codegen::Backend) -> Result<()> {
    codegen::emit_with_backend(ir, backend)?;
    Ok(())
}
