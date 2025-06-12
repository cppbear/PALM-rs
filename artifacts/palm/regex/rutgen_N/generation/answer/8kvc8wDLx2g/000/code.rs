// Answer 0

#[test]
fn test_allow_invalid_utf8_true() {
    struct TranslatorBuilder {
        allow_invalid_utf8: bool,
    }

    let mut builder = TranslatorBuilder { allow_invalid_utf8: false };
    builder.allow_invalid_utf8(true);
    assert!(builder.allow_invalid_utf8);
}

#[test]
fn test_allow_invalid_utf8_false() {
    struct TranslatorBuilder {
        allow_invalid_utf8: bool,
    }

    let mut builder = TranslatorBuilder { allow_invalid_utf8: true };
    builder.allow_invalid_utf8(false);
    assert!(!builder.allow_invalid_utf8);
}

