// Answer 0

#[test]
fn test_unicode_enable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        unicode: Option<bool>,
    }

    let mut builder = TranslatorBuilder {
        flags: Flags { unicode: None },
    };

    builder.unicode(true);
    assert_eq!(builder.flags.unicode, None);
}

#[test]
fn test_unicode_disable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        unicode: Option<bool>,
    }

    let mut builder = TranslatorBuilder {
        flags: Flags { unicode: None },
    };

    builder.unicode(false);
    assert_eq!(builder.flags.unicode, Some(false));
}

