// Answer 0

#[test]
fn test_allow_invalid_utf8_true() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.allow_invalid_utf8(true);
    assert_eq!(result.allow_invalid_utf8, true);
}

#[test]
fn test_allow_invalid_utf8_false() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.allow_invalid_utf8(false);
    assert_eq!(result.allow_invalid_utf8, false);
}

#[test]
fn test_allow_invalid_utf8_multiple_calls() {
    let mut builder = TranslatorBuilder::new();
    builder.allow_invalid_utf8(true);
    let result = builder.allow_invalid_utf8(false);
    assert_eq!(result.allow_invalid_utf8, false);
}

