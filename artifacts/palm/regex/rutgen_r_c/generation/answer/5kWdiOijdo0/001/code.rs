// Answer 0

#[test]
fn test_class_unicode_range_new_valid_range() {
    let range = ClassUnicodeRange::new('a', 'z');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'z');

    let range = ClassUnicodeRange::new('A', 'Z');
    assert_eq!(range.start(), 'A');
    assert_eq!(range.end(), 'Z');

    let range = ClassUnicodeRange::new('0', '9');
    assert_eq!(range.start(), '0');
    assert_eq!(range.end(), '9');

    let range = ClassUnicodeRange::new('\u{0000}', '\u{FFFF}');
    assert_eq!(range.start(), '\u{0000}');
    assert_eq!(range.end(), '\u{FFFF}');
}

#[test]
fn test_class_unicode_range_new_reverse_range() {
    let range = ClassUnicodeRange::new('z', 'a');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'z');
}

#[test]
fn test_class_unicode_range_new_equal_chars() {
    let range = ClassUnicodeRange::new('c', 'c');
    assert_eq!(range.start(), 'c');
    assert_eq!(range.end(), 'c');
}

#[test]
#[should_panic]
fn test_class_unicode_range_new_panic_on_invalid() {
    ClassUnicodeRange::new('\u{FFFF}', '\u{0000}');
}

