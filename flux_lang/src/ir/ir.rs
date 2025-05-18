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
