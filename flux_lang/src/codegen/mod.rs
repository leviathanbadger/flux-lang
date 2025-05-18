//! Code generation stubs

use crate::ir::IrModule;

/// Available code generation backends.
#[derive(Clone, Copy, Debug)]
pub enum Backend {
    Llvm,
    Cranelift,
    Wasm,
}

pub mod cranelift;
pub mod llvm;
pub mod wasm;

pub fn emit(ir: &IrModule) -> Result<(), String> {
    emit_with_backend(ir, Backend::Llvm)
}

/// Emit code for the given IR using the selected backend.
pub fn emit_with_backend(ir: &IrModule, backend: Backend) -> Result<(), String> {
    match backend {
        Backend::Llvm => llvm::emit_llvm(ir),
        Backend::Cranelift => cranelift::emit_cranelift(ir),
        Backend::Wasm => wasm::emit_wasm(ir),
    }
}
