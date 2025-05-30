use flux_lang::{compile, plugins};

#[test]
fn typechecker_stub() {
    plugins::REGISTRY.clear();
    assert!(compile("dummy").is_ok());
}
