use flux_lang::{compile, plugins};

#[test]
fn typechecker_stub() {
    plugins::clear_plugins();
    assert!(compile("dummy").is_ok());
}
