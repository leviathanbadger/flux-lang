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

#[test]
fn parse_error_reports_location() {
    plugins::clear_plugins();
    let err = parse_program("1").unwrap_err();
    let pe = err
        .downcast_ref::<flux_lang::syntax::ParseError>()
        .expect("ParseError");
    assert_eq!(pe.line, 0);
    assert_eq!(pe.column, 0);
    assert_eq!(pe.message, "invalid token");
}
