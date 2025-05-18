//! Core compiler library for FluxLang.

pub mod codegen;
pub mod ir;
pub mod macros;
pub mod plugins;
pub mod semantic;
pub mod syntax;

/// Stub compile entry point.
pub fn compile(source: &str) -> Result<(), String> {
    // Parse source into AST
    let mut ast = syntax::grammar::ProgramParser::new()
        .parse(source)
        .map_err(|e| format!("parse error: {e}"))?;

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
    codegen::emit(&ir)?;
    Ok(())
}
