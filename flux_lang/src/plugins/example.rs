use super::Plugin;
use crate::syntax::ast::Program;

/// Simple plugin that dumps the AST for debugging.
pub struct DumpAstPlugin;

impl Plugin for DumpAstPlugin {
    fn run(&self, program: &mut Program) {
        println!("[plugin] AST: {:?}", program);
    }
}
