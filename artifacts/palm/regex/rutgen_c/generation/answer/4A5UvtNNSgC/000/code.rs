// Answer 0

#[test]
fn test_new_parser_builder() {
    let builder = ParserBuilder::new();
    assert_eq!(builder.ignore_whitespace, false);
    assert_eq!(builder.nest_limit, 0);
    assert_eq!(builder.octal, false);
}

#[test]
fn test_parser_builder_default() {
    let builder = ParserBuilder::default();
    assert_eq!(builder.ignore_whitespace, false);
    assert_eq!(builder.nest_limit, 0);
    assert_eq!(builder.octal, false);
}

