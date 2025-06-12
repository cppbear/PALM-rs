// Answer 0

#[test]
fn test_case_insensitive_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.case_insensitive(true);
    // Assert that case insensitivity has been set correctly.
    assert_eq!(parser_builder.hir.flags.case_insensitive, Some(true));
}

#[test]
fn test_case_insensitive_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.case_insensitive(false);
    // Assert that case insensitivity has been set to None.
    assert_eq!(parser_builder.hir.flags.case_insensitive, None);
}

#[test]
fn test_case_insensitive_multiple_calls() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.case_insensitive(true);
    parser_builder.case_insensitive(false);
    // Assert that case insensitivity is None after disabling it.
    assert_eq!(parser_builder.hir.flags.case_insensitive, None);
}

