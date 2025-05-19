//! Utility functions for syntax operations.

/// Convert a character offset within `src` to a zero-based (line, column) pair.
///
/// Lines are separated by `\n` characters.
pub fn offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 0usize;
    let mut col = 0usize;
    for (i, ch) in src.char_indices() {
        if i == offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

#[cfg(test)]
mod tests {
    use super::offset_to_line_col;

    #[test]
    fn calculates_first_line() {
        assert_eq!(offset_to_line_col("abc", 2), (0, 2));
    }

    #[test]
    fn calculates_multiline() {
        let src = "a\nbc\nde";
        assert_eq!(offset_to_line_col(src, 0), (0, 0));
        assert_eq!(offset_to_line_col(src, 2), (1, 0));
        assert_eq!(offset_to_line_col(src, 5), (2, 0));
    }
}
