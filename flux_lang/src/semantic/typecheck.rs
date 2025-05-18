use super::env::Env;
use crate::syntax::ast::Program;
use anyhow::Result;

pub fn check(_program: &Program) -> Result<()> {
    let _env = Env::new();
    // TODO: implement type checking
    Ok(())
}
