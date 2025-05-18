use flux_lang::compile;

#[test]
fn compile_empty_source() {
    assert!(compile("").is_ok());
}
