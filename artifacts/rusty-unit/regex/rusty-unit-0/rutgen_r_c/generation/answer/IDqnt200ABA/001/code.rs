// Answer 0

#[test]
fn test_case_insensitive_enable() {
    let mut parser_builder = ParserBuilder::new();
    let result = parser_builder.case_insensitive(true);
    assert_eq!(result, &mut parser_builder);
}

#[test]
fn test_case_insensitive_disable() {
    let mut parser_builder = ParserBuilder::new();
    let result = parser_builder.case_insensitive(false);
    assert_eq!(result, &mut parser_builder);
}

#[test]
fn test_case_insensitive_multiple_calls() {
    let mut parser_builder = ParserBuilder::new();
    let result = parser_builder.case_insensitive(true).case_insensitive(false);
    assert_eq!(result, &mut parser_builder);
}

