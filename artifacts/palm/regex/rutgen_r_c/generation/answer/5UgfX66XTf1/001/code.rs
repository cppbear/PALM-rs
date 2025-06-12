// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(true);
    
    assert!(parser_builder.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.ignore_whitespace(false);
    
    assert!(!parser_builder.ignore_whitespace);
} 

#[test]
fn test_ignore_whitespace_chain() {
    let mut parser_builder = ParserBuilder::new();
    let returned_builder = parser_builder.ignore_whitespace(true).ignore_whitespace(false);
    
    assert_eq!(returned_builder, &parser_builder);
    assert!(!parser_builder.ignore_whitespace);
}

