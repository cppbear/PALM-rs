// Answer 0

#[test]
fn test_swap_greed_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.hir = TranslatorBuilder::new();
    let result = parser_builder.swap_greed(true);
    assert_eq!(result.hir.flags.swap_greed, Some(true));
}

#[test]
fn test_swap_greed_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.hir = TranslatorBuilder::new();
    let result = parser_builder.swap_greed(false);
    assert_eq!(result.hir.flags.swap_greed, None);
}

#[test]
fn test_swap_greed_multiple_calls() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.hir = TranslatorBuilder::new();
    
    // Enable swap greed and check
    let result_enable = parser_builder.swap_greed(true);
    assert_eq!(result_enable.hir.flags.swap_greed, Some(true));

    // Disable swap greed and check
    let result_disable = result_enable.swap_greed(false);
    assert_eq!(result_disable.hir.flags.swap_greed, None);
}

