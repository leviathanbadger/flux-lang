use crate::syntax::ast::Program;

/// Expand macros in the given program.
pub fn expand(program: &mut Program) {
    let _ = program; // suppress unused variable warning
                     // TODO: implement hygienic macro expansion
}
