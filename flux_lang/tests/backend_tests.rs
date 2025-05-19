use flux_lang::{codegen::Backend, compile_with_backend, plugins};

#[test]
fn compile_with_all_backends() {
    plugins::clear_plugins();
    for backend in [Backend::Llvm, Backend::Cranelift, Backend::Wasm] {
        assert!(compile_with_backend("", backend).is_ok());
    }
}
