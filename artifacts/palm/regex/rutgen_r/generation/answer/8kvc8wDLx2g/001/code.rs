// Answer 0

#[test]
fn test_allow_invalid_utf8_true() {
    struct TranslatorBuilder {
        allow_invalid_utf8: bool,
    }

    let mut builder = TranslatorBuilder { allow_invalid_utf8: false };
    let result = builder.allow_invalid_utf8(true);
    assert_eq!(result.allow_invalid_utf8, true);
}

#[test]
fn test_allow_invalid_utf8_false() {
    struct TranslatorBuilder {
        allow_invalid_utf8: bool,
    }

    let mut builder = TranslatorBuilder { allow_invalid_utf8: true };
    let result = builder.allow_invalid_utf8(false);
    assert_eq!(result.allow_invalid_utf8, false);
}

#[test]
fn test_allow_invalid_utf8_initial_state() {
    struct TranslatorBuilder {
        allow_invalid_utf8: bool,
    }

    let builder = TranslatorBuilder { allow_invalid_utf8: false };
    // No state change here as we're not mutating
    assert_eq!(builder.allow_invalid_utf8, false);
}

