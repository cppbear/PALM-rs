// Answer 0

#[test]
fn test_parser_builder_default() {
    let builder = ParserBuilder::default();
    let parser = builder.build();
    assert_eq!(parser.nest_limit, 0);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.ignore_whitespace.get(), false);
}

#[test]
fn test_parser_builder_custom_nest_limit() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(10);
    let parser = builder.build();
    assert_eq!(parser.nest_limit, 10);
}

#[test]
fn test_parser_builder_custom_octal() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    let parser = builder.build();
    assert_eq!(parser.octal, true);
}

#[test]
fn test_parser_builder_ignore_whitespace() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
    let parser = builder.build();
    assert_eq!(parser.ignore_whitespace.get(), true);
}

#[test]
fn test_parser_builder_comments() {
    let builder = ParserBuilder::default();
    let parser = builder.build();
    assert_eq!(parser.comments.borrow().len(), 0);
}

