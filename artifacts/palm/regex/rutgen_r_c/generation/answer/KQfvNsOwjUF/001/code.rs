// Answer 0

#[test]
fn test_multi_line_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.multi_line(true);
    let translator_builder = parser_builder.hir.clone();
    assert_eq!(translator_builder.flags.multi_line, Some(true));
}

#[test]
fn test_multi_line_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.multi_line(false);
    let translator_builder = parser_builder.hir.clone();
    assert_eq!(translator_builder.flags.multi_line, None);
}

#[test]
fn test_multi_line_multiple_calls() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.multi_line(true);
    let translator_builder = parser_builder.hir.clone();
    assert_eq!(translator_builder.flags.multi_line, Some(true));

    parser_builder.multi_line(false);
    let translator_builder = parser_builder.hir.clone();
    assert_eq!(translator_builder.flags.multi_line, None);
}

