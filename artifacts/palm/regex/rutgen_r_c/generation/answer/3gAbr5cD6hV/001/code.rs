// Answer 0

#[test]
fn test_new_empty_ranges() {
    let ranges: Vec<ClassUnicodeRange> = vec![];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(class_unicode.ranges().len(), 0);
}

#[test]
fn test_new_single_range() {
    let ranges = vec![ClassUnicodeRange { start: 'a', end: 'b' }];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(class_unicode.ranges().len(), 1);
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: 'a', end: 'b' });
}

#[test]
fn test_new_multiple_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'e', end: 'g' }
    ];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(class_unicode.ranges().len(), 2);
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: 'a', end: 'c' });
    assert_eq!(class_unicode.ranges()[1], ClassUnicodeRange { start: 'e', end: 'g' });
}

#[test]
fn test_new_overlapping_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'e' },
        ClassUnicodeRange { start: 'c', end: 'f' }
    ];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(class_unicode.ranges().len(), 2);
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: 'a', end: 'e' });
    assert_eq!(class_unicode.ranges()[1], ClassUnicodeRange { start: 'c', end: 'f' });
}

#[test]
fn test_new_identical_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'x', end: 'y' },
        ClassUnicodeRange { start: 'x', end: 'y' }
    ];
    let class_unicode = ClassUnicode::new(ranges);
    assert_eq!(class_unicode.ranges().len(), 2);
    assert_eq!(class_unicode.ranges()[0], ClassUnicodeRange { start: 'x', end: 'y' });
    assert_eq!(class_unicode.ranges()[1], ClassUnicodeRange { start: 'x', end: 'y' });
}

