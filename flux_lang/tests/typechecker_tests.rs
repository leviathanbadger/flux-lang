use flux_lang::compile;

#[test]
fn typechecker_stub() {
    assert!(compile("dummy").is_ok());
}
