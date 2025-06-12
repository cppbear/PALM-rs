// Answer 0

#[test]
fn test_parser_builder_octal_enabled() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(true);
    assert!(parser_builder.ast.octal);
}

#[test]
fn test_parser_builder_octal_disabled() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(false);
    assert!(!parser_builder.ast.octal);
}

#[test]
fn test_parser_builder_octal_multiple_calls() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(true).octal(false);
    assert!(!parser_builder.ast.octal);
}

#[test]
#[should_panic]
fn test_parser_builder_octal_uninitialized() {
    let mut parser_builder: ParserBuilder = ParserBuilder::default();
    parser_builder.octal(false);
    assert!(!parser_builder.ast.octal);
}

