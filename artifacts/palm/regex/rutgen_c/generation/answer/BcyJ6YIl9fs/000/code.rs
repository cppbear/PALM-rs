// Answer 0

#[test]
fn test_is_always_utf8_unicode() {
    let class = Class::Unicode(ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::SomeKind, // Replace SomeKind with an actual kind
    });
    assert!(class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_bytes_all_ascii() {
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(0x00, 0x7F)]);
    let class = Class::Bytes(class_bytes);
    assert!(class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_bytes_non_ascii() {
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(0x80, 0xFF)]);
    let class = Class::Bytes(class_bytes);
    assert!(!class.is_always_utf8());
}

