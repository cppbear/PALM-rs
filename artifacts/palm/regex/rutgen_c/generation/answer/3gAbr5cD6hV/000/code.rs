// Answer 0

#[test]
fn test_class_unicode_new_with_non_overlapping_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'd', end: 'e' },
    ];
    let class_unicode = ClassUnicode::new(ranges);

    assert_eq!(class_unicode.ranges().len(), 2);
}

#[test]
fn test_class_unicode_new_with_overlapping_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'b', end: 'd' },
    ];
    let class_unicode = ClassUnicode::new(ranges);

    assert_eq!(class_unicode.ranges().len(), 2);
}

#[test]
fn test_class_unicode_new_with_single_range() {
    let ranges = vec![
        ClassUnicodeRange { start: 'x', end: 'y' },
    ];
    let class_unicode = ClassUnicode::new(ranges);

    assert_eq!(class_unicode.ranges().len(), 1);
}

#[test]
fn test_class_unicode_new_with_empty_ranges() {
    let ranges: Vec<ClassUnicodeRange> = vec![];
    let class_unicode = ClassUnicode::new(ranges);

    assert_eq!(class_unicode.ranges().len(), 0);
}

