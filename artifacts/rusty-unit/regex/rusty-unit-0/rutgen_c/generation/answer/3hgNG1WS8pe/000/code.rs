// Answer 0

#[test]
fn test_parser_builder_new() {
    let builder = ParserBuilder::new();
    assert_eq!(builder.ignore_whitespace, false);
    assert_eq!(builder.nest_limit, 250);
    assert_eq!(builder.octal, false);
}

#[test]
fn test_parser_builder_nest_limit() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(100);
    assert_eq!(builder.nest_limit, 100);
}

#[test]
fn test_parser_builder_octal() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    assert_eq!(builder.octal, true);
}

#[test]
fn test_parser_builder_ignore_whitespace() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
    assert_eq!(builder.ignore_whitespace, true);
}

