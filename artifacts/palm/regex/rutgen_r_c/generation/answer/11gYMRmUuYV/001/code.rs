// Answer 0

#[test]
fn test_unicode_with_some_value() {
    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };
    assert_eq!(flags.unicode(), false);
}

#[test]
fn test_unicode_with_none_value() {
    let flags = Flags {
        unicode: None,
        ..Flags::default()
    };
    assert_eq!(flags.unicode(), true);
}

#[test]
fn test_unicode_with_some_value_true() {
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    assert_eq!(flags.unicode(), true);
}

