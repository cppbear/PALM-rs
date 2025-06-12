// Answer 0

#[test]
fn test_new_parser_builder() {
    let builder = regex_syntax::ast::parse::new();
    assert_eq!(builder.ignore_whitespace, false);
    assert_eq!(builder.nest_limit, 250);
    assert_eq!(builder.octal, false);
}

