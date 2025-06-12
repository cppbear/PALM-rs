// Answer 0

#[test]
fn test_ranges_single_range() {
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    let interval_set = IntervalSet::new(vec![range]);
    let class_unicode = ClassUnicode { set: interval_set };
    
    let expected = vec![ClassUnicodeRange { start: 'a', end: 'z' }];
    assert_eq!(class_unicode.ranges(), expected);
}

#[test]
fn test_ranges_multiple_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'd', end: 'e' },
    ];
    
    let interval_set = IntervalSet::new(ranges);
    let class_unicode = ClassUnicode { set: interval_set };
    
    let expected = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'd', end: 'e' },
    ];
    assert_eq!(class_unicode.ranges(), expected);
}

#[test]
fn test_ranges_empty() {
    let interval_set = IntervalSet::new(vec![]);
    let class_unicode = ClassUnicode { set: interval_set };
    
    let expected: Vec<ClassUnicodeRange> = vec![];
    assert_eq!(class_unicode.ranges(), expected);
}

#[test]
fn test_ranges_single_character() {
    let range = ClassUnicodeRange { start: 'a', end: 'a' };
    let interval_set = IntervalSet::new(vec![range]);
    let class_unicode = ClassUnicode { set: interval_set };
    
    let expected = vec![ClassUnicodeRange { start: 'a', end: 'a' }];
    assert_eq!(class_unicode.ranges(), expected);
}

#[test]
fn test_ranges_non_contiguous_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'A', end: 'A' },
        ClassUnicodeRange { start: 'Z', end: 'Z' },
    ];
    
    let interval_set = IntervalSet::new(ranges);
    let class_unicode = ClassUnicode { set: interval_set };
    
    let expected = vec![
        ClassUnicodeRange { start: 'A', end: 'A' },
        ClassUnicodeRange { start: 'Z', end: 'Z' },
    ];
    assert_eq!(class_unicode.ranges(), expected);
}

