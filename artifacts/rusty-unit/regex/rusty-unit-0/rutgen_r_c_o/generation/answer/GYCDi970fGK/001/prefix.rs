// Answer 0

#[test]
fn test_swap_greed_true() {
    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_false() {
    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(false);
}

