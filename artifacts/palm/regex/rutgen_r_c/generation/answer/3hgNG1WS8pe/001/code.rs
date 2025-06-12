// Answer 0

#[test]
fn test_parser_builder_new() {
    let builder = ParserBuilder::new();
    assert_eq!(builder.ignore_whitespace, false);
    assert_eq!(builder.nest_limit, 250);
    assert_eq!(builder.octal, false);
}

