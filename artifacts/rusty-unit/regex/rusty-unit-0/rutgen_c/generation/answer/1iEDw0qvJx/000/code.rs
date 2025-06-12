// Answer 0

#[test]
fn test_parser_new() {
    let parser = Parser::new();
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());
    assert_eq!(parser.nest_limit, 256); // Assuming default limit is 256, adjust according to the actual implementation
    assert_eq!(parser.octal, false); // Assuming octal support is off by default
    assert_eq!(parser.ignore_whitespace.get(), false); // Assuming ignore_whitespace is false by default
}

#[test]
fn test_parser_builder_new() {
    let builder = ParserBuilder::new();
    assert_eq!(builder.ast, ast::parse::ParserBuilder::default()); // Adjust according to actual defaults
    assert_eq!(builder.hir, hir::translate::TranslatorBuilder::default()); // Adjust according to actual defaults
}

#[test]
fn test_parser_builder_build() {
    let builder = ParserBuilder::new()
        .nest_limit(128)
        .octal(true)
        .ignore_whitespace(true);
    let parser = builder.build();
    assert_eq!(parser.nest_limit, 128);
    assert_eq!(parser.octal, true);
    assert_eq!(parser.ignore_whitespace.get(), true);
}

