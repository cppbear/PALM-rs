// Answer 0

#[test]
fn test_translator_new() {
    let translator = Translator::new();
    assert!(translator.allow_invalid_utf8 == false);
    assert_eq!(translator.flags.get(), Flags::default());
    assert!(translator.stack.borrow().is_empty());
}

#[test]
fn test_translator_builder_new() {
    let builder = TranslatorBuilder::new();
    assert_eq!(builder.allow_invalid_utf8, false);
    assert_eq!(builder.flags, Flags::default());
}

#[test]
fn test_translator_builder_reverse_build() {
    let builder = TranslatorBuilder::new();
    let translator = builder.build();
    assert!(translator.allow_invalid_utf8 == false);
    assert_eq!(translator.flags.get(), Flags::default());
    assert!(translator.stack.borrow().is_empty());
}

