// Answer 0

#[test]
fn test_class_unicode_range_creation_valid() {
    let range = ClassUnicodeRange::new('a', 'z');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'z');
}

#[test]
fn test_class_unicode_range_creation_reversed() {
    let range = ClassUnicodeRange::new('z', 'a');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'z');
}

#[test]
fn test_class_unicode_range_creation_equal() {
    let range = ClassUnicodeRange::new('a', 'a');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'a');
}

#[test]
fn test_class_unicode_range_creation_boundary() {
    let range = ClassUnicodeRange::new(char::from_u32(0).unwrap(), char::from_u32(u32::MAX).unwrap());
    assert_eq!(range.start(), char::from_u32(0).unwrap());
    assert_eq!(range.end(), char::from_u32(u32::MAX).unwrap());
}

