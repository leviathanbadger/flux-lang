//! Core compiler library for FluxLang.

pub mod codegen;
pub mod ir;
pub mod semantic;
pub mod syntax;

/// Stub compile entry point.
pub fn compile(source: &str) -> Result<(), String> {
    // Parse source into AST
    let ast = syntax::grammar::ProgramParser::new()
        .parse(source)
        .map_err(|e| format!("parse error: {e}"))?;

    // Type check
    semantic::check(&ast)?;

    // Lower to IR and optimize
    let mut ir = ir::lower(&ast);
    ir::run_passes(&mut ir);

    // Emit code (placeholder)
    codegen::emit(&ir)?;
    Ok(())
}
