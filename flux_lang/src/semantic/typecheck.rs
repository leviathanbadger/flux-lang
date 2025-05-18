use super::env::Env;
use crate::syntax::ast::Program;

pub fn check(_program: &Program) -> Result<(), String> {
    let _env = Env::new();
    // TODO: implement type checking
    Ok(())
}
