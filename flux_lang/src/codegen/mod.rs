//! Code generation stubs

use crate::ir::IrModule;

pub mod cranelift;
pub mod llvm;
pub mod wasm;

pub fn emit(ir: &IrModule) -> Result<(), String> {
    // TODO: choose backend via options
    llvm::emit_llvm(ir)
}
