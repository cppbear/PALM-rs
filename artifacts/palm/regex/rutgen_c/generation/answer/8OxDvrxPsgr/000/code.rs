// Answer 0

#[test]
fn test_unicodes_flag_enabled() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(true);
    assert_eq!(builder.flags.unicode, None);
}

#[test]
fn test_unicodes_flag_disabled() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(false);
    assert_eq!(builder.flags.unicode, Some(false));
}

#[test]
fn test_unicodes_flag_stays_same_on_multiple_calls() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(true);
    let original_unicode = builder.flags.unicode;
    builder.unicode(true); // Call again with same value
    assert_eq!(builder.flags.unicode, original_unicode);
    builder.unicode(false); // Change to false now
    assert_eq!(builder.flags.unicode, Some(false));
}

