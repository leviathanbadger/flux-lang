use flux_lang::{compile, parse_program, plugins};

#[test]
fn compile_empty_source() {
    plugins::clear_plugins();
    assert!(compile("").is_ok());
}

#[test]
fn parse_returns_ast() {
    plugins::clear_plugins();
    let ast = parse_program("").expect("parse failure");
    assert_eq!(format!("{ast:?}"), "Program");
}
