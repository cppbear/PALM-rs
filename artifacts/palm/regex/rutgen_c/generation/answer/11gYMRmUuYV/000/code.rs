// Answer 0

#[test]
fn test_unicode_default() {
    let flags = Flags::default();
    assert!(flags.unicode());
}

#[test]
fn test_unicode_some_true() {
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    assert!(flags.unicode());
}

#[test]
fn test_unicode_some_false() {
    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };
    assert!(!flags.unicode());
}

