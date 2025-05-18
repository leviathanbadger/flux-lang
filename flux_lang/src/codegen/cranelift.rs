use crate::ir::IrModule;
use anyhow::Result;

pub fn emit_cranelift(_ir: &IrModule) -> Result<()> {
    // TODO: implement Cranelift JIT backend
    Ok(())
}
