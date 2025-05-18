use flux_lang::{codegen::Backend, compile_with_backend};

#[test]
fn compile_with_all_backends() {
    for backend in [Backend::Llvm, Backend::Cranelift, Backend::Wasm] {
        assert!(compile_with_backend("", backend).is_ok());
    }
}
