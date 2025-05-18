use flux_lang::{compile, parse_program};

#[test]
fn compile_empty_source() {
    assert!(compile("").is_ok());
}

#[test]
fn parse_returns_ast() {
    let ast = parse_program("").expect("parse failure");
    assert_eq!(format!("{ast:?}"), "Program");
}
