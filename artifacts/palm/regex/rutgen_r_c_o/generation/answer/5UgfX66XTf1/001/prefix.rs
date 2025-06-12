// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_false() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(false);
}

