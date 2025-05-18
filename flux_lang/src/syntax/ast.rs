//! Abstract syntax tree definitions for FluxLang.

#[derive(Debug, PartialEq)]
pub struct Program;

#[cfg(test)]
mod tests {
    use super::Program;

    #[test]
    fn debug_format() {
        let prog = Program;
        assert_eq!(format!("{prog:?}"), "Program");
    }
}
