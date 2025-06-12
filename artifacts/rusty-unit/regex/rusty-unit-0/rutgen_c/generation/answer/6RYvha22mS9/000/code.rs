// Answer 0

#[test]
fn test_translator_builder_new() {
    let builder = TranslatorBuilder::new();
    assert_eq!(builder.allow_invalid_utf8, false);
    assert_eq!(builder.flags, Flags::default());
}

