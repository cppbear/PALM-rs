// Answer 0

#[test]
fn test_push_valid_range() {
    let mut class_unicode = ClassUnicode::empty();
    let valid_range = ClassUnicodeRange { start: 'a', end: 'z' };
    class_unicode.push(valid_range);
    assert_eq!(class_unicode.ranges(), &[valid_range]);
}

#[test]
fn test_push_empty_range() {
    let mut class_unicode = ClassUnicode::empty();
    let empty_range = ClassUnicodeRange { start: 'a', end: 'a' };
    class_unicode.push(empty_range);
    assert_eq!(class_unicode.ranges(), &[empty_range]);
}

#[test]
fn test_push_ordered_ranges() {
    let mut class_unicode = ClassUnicode::empty();
    let range1 = ClassUnicodeRange { start: 'a', end: 'd' };
    let range2 = ClassUnicodeRange { start: 'e', end: 'h' };
    class_unicode.push(range1);
    class_unicode.push(range2);
    assert_eq!(class_unicode.ranges(), &[range1, range2]);
}

#[test]
#[should_panic]
fn test_push_invalid_range() {
    let mut class_unicode = ClassUnicode::empty();
    let invalid_range = ClassUnicodeRange { start: 'z', end: 'a' }; // Invalid range: start > end
    class_unicode.push(invalid_range); // This should panic
}

#[test]
fn test_push_overlapping_ranges() {
    let mut class_unicode = ClassUnicode::empty();
    let range1 = ClassUnicodeRange { start: 'a', end: 'c' };
    let range2 = ClassUnicodeRange { start: 'b', end: 'd' }; // Overlapping range
    class_unicode.push(range1);
    class_unicode.push(range2);
    assert_eq!(class_unicode.ranges(), &[range1, range2]); // Ensure both ranges are present
}

