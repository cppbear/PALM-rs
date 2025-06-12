// Answer 0

#[test]
fn test_swap_greed_enabled() {
    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(true);
    assert_eq!(builder.flags.swap_greed, Some(true));
}

#[test]
fn test_swap_greed_disabled() {
    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(false);
    assert_eq!(builder.flags.swap_greed, None);
}

