// Answer 0

#[test]
fn test_swap_greed_default() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.swap_greed(false);
    assert_eq!(parser_builder.hir.flags.swap_greed, None);
}

#[test]
fn test_swap_greed_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.swap_greed(true);
    assert_eq!(parser_builder.hir.flags.swap_greed, Some(true));
}

#[test]
fn test_swap_greed_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.swap_greed(true); // First enable it
    parser_builder.swap_greed(false); // Then disable it
    assert_eq!(parser_builder.hir.flags.swap_greed, None);
}

