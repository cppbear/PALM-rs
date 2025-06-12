// Answer 0

#[test]
fn test_is_always_utf8_unicode_class() {
    let unicode_class = Class::Unicode(ClassUnicode {
        span: Span { start: 0, end: 0 }, // Assuming a default Span structure
        negated: false, // Not negated
        kind: ClassUnicodeKind::SomeKind, // Placeholder for a valid kind
    });

    assert!(unicode_class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_bytes_ascii() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0x00, 0x7F), // ASCII range
    ]));

    assert!(bytes_class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_bytes_non_ascii() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0x80, 0xFF), // Non-ASCII range
    ]));

    assert!(!bytes_class.is_always_utf8());
}

