// Answer 0

#[test]
fn test_allow_invalid_utf8_enabled() {
    let mut builder = TranslatorBuilder::new();
    builder.allow_invalid_utf8(true);
    assert!(builder.allow_invalid_utf8);
}

#[test]
fn test_allow_invalid_utf8_disabled() {
    let mut builder = TranslatorBuilder::new();
    builder.allow_invalid_utf8(false);
    assert!(!builder.allow_invalid_utf8);
}

