//! Intermediate representation definitions

use crate::syntax::ast::Program;
use petgraph::graph::Graph;

pub struct IrModule {
    pub graph: Graph<(), ()>,
}

pub fn lower(_program: &Program) -> IrModule {
    // TODO: implement lowering from AST to IR
    IrModule {
        graph: Graph::new(),
    }
}

pub mod opt {
    use super::IrModule;

    pub fn run_passes(_ir: &mut IrModule) {
        // TODO: implement optimization passes
    }
}
