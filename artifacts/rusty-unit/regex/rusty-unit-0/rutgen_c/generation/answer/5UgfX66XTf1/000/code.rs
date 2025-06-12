// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(true);
    assert_eq!(parser_builder.ast.ignore_whitespace, true);
}

#[test]
fn test_ignore_whitespace_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(false);
    assert_eq!(parser_builder.ast.ignore_whitespace, false);
}

#[test]
fn test_ignore_whitespace_chain() {
    let mut parser_builder = ParserBuilder::new();
    let result = parser_builder.ignore_whitespace(true).ignore_whitespace(false);
    assert_eq!(parser_builder.ast.ignore_whitespace, false);
    assert_eq!(result.ast.ignore_whitespace, false);
}

