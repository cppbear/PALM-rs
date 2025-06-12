// Answer 0

#[test]
fn test_class_unicode_negate_empty() {
    let mut class_unicode = ClassUnicode::empty();
    class_unicode.negate();
    assert_eq!(class_unicode.ranges().len(), 1); // Checking that it has negated to a full range
    assert_eq!(class_unicode.ranges()[0].start, char::MIN);
    assert_eq!(class_unicode.ranges()[0].end, char::MAX);
}

#[test]
fn test_class_unicode_negate_single_range() {
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    let mut class_unicode = ClassUnicode::new(vec![range]);
    class_unicode.negate();
    assert_eq!(class_unicode.ranges().len(), 1); // Should still have one range after negation
    assert_eq!(class_unicode.ranges()[0].start, char::MIN);
    assert_eq!(class_unicode.ranges()[0].end, 'a' as u32 - 1);
    assert_eq!(class_unicode.ranges()[1].start, 'z' as u32 + 1);
    assert_eq!(class_unicode.ranges()[1].end, char::MAX);
}

#[test]
fn test_class_unicode_negate_multiple_ranges() {
    let range1 = ClassUnicodeRange { start: 'a', end: 'c' };
    let range2 = ClassUnicodeRange { start: 'e', end: 'g' };
    let mut class_unicode = ClassUnicode::new(vec![range1, range2]);
    class_unicode.negate();
    // After negation, we should have ranges that exclude the originally specified ranges
    assert_eq!(class_unicode.ranges().len(), 3); // Should have three ranges after negation

    assert_eq!(class_unicode.ranges()[0].start, char::MIN);
    assert_eq!(class_unicode.ranges()[0].end, 'a' as u32 - 1);
    
    assert_eq!(class_unicode.ranges()[1].start, 'c' as u32 + 1);
    assert_eq!(class_unicode.ranges()[1].end, 'e' as u32 - 1);

    assert_eq!(class_unicode.ranges()[2].start, 'g' as u32 + 1);
    assert_eq!(class_unicode.ranges()[2].end, char::MAX);
}

