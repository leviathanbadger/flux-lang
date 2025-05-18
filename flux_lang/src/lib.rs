//! Core compiler library for FluxLang.

pub mod codegen;
pub mod ir;
pub mod macros;
pub mod plugins;
pub mod semantic;
pub mod syntax;

use anyhow::{anyhow, Result};

/// Stub compile entry point.
pub fn compile(source: &str) -> Result<()> {
    compile_with_backend(source, codegen::Backend::Llvm)
}

/// Parse FluxLang source into an AST.
pub fn parse_program(source: &str) -> Result<syntax::ast::Program> {
    syntax::grammar::ProgramParser::new()
        .parse(source)
        .map_err(|e| anyhow!("parse error: {e}"))
}

/// Compile FluxLang source using the specified backend.
pub fn compile_with_backend(source: &str, backend: codegen::Backend) -> Result<()> {
    // Parse source into AST
    let mut ast = parse_program(source)?;

    // Expand macros
    macros::expand(&mut ast);

    // Register and run development plugins
    plugins::register_default_plugins();
    plugins::run_all(&mut ast);

    // Type check
    semantic::check(&ast)?;

    // Lower to IR and optimize
    let mut ir = ir::lower(&ast);
    ir::run_passes(&mut ir);

    // Emit code (placeholder)
    codegen::emit_with_backend(&ir, backend)?;
    Ok(())
}
