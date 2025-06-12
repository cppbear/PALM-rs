// Answer 0

#[test]
fn test_negate_empty_class_unicode() {
    let mut class_unicode = ClassUnicode::empty();
    class_unicode.negate();
    assert_eq!(class_unicode.ranges().len(), 1); // Expecting a full range after negation
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: char::from(0), end: char::from(u32::max_value() as u8) });
}

#[test]
fn test_negate_single_range() {
    let mut class_unicode = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    class_unicode.negate();
    assert_eq!(class_unicode.ranges().len(), 3); // Expecting three ranges: below 'a', between 'z' and 'max', above 'max'
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: char::from(0), end: 'a' });
    assert_eq!(class_unicode.ranges()[1], ClassUnicodeRange { start: 'z', end: char::from(u32::max_value() as u8) });
}

#[test]
fn test_negate_multiple_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'd', end: 'e' },
    ]);
    class_unicode.negate();
    assert_eq!(class_unicode.ranges().len(), 4); // Expecting ranges: 0 to 'a', 'b' to 'd', 'e' to 'max'
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: char::from(0), end: 'a' });
    assert_eq!(class_unicode.ranges()[1], ClassUnicodeRange { start: 'b', end: 'd' });
    assert_eq!(class_unicode.ranges()[2], ClassUnicodeRange { start: 'e', end: char::from(u32::max_value() as u8) });
}

