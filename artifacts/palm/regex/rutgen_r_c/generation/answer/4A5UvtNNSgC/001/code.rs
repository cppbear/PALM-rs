// Answer 0

#[test]
fn test_new_parser_builder() {
    let parser_builder = ParserBuilder::new();
    
    assert_eq!(parser_builder.ignore_whitespace, false);
    assert_eq!(parser_builder.nest_limit, 256); // Assuming default is 256
    assert_eq!(parser_builder.octal, false);
}

#[test]
fn test_default_parser_builder_properties() {
    let parser_builder = ParserBuilder::default();
    
    assert!(parser_builder == ParserBuilder::new()); // Should be equal to newly created instance
}

