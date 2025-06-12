// Answer 0

#[test]
fn test_parser_builder_new() {
    let builder = ParserBuilder::new();
    let parser = builder.build();
    assert_eq!(parser.pos.get(), Position { offset: 0, line: 1, column: 1 });
    assert_eq!(parser.capture_index.get(), 0);
}

#[test]
fn test_parser_builder_with_nest_limit() {
    let mut builder = ParserBuilder::new().nest_limit(10);
    let parser = builder.build();
    assert_eq!(parser.nest_limit, 10);
}

#[test]
fn test_parser_builder_with_octal() {
    let mut builder = ParserBuilder::new().octal(true);
    let parser = builder.build();
    assert_eq!(parser.octal, true);

    let mut builder_no_octal = ParserBuilder::new().octal(false);
    let parser_no_octal = builder_no_octal.build();
    assert_eq!(parser_no_octal.octal, false);
}

#[test]
fn test_parser_builder_with_ignore_whitespace() {
    let mut builder = ParserBuilder::new().ignore_whitespace(true);
    let parser = builder.build();
    assert_eq!(parser.ignore_whitespace.get(), true);

    let mut builder_no_ignore = ParserBuilder::new().ignore_whitespace(false);
    let parser_no_ignore = builder_no_ignore.build();
    assert_eq!(parser_no_ignore.ignore_whitespace.get(), false);
}

