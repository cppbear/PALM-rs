// Answer 0

#[test]
fn test_translator_builder_new() {
    let builder = TranslatorBuilder::new();

    assert_eq!(builder.allow_invalid_utf8, false);
    assert_eq!(builder.flags, Flags::default());
}

#[test]
fn test_translator_builder_flags_default() {
    let flags = Flags::default();

    assert_eq!(flags.case_insensitive, None);
    assert_eq!(flags.multi_line, None);
    assert_eq!(flags.dot_matches_new_line, None);
    assert_eq!(flags.swap_greed, None);
    assert_eq!(flags.unicode, None);
}

