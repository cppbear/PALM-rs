// Answer 0

#[test]
fn test_ranges_with_non_empty_set() {
    let range1 = ClassUnicodeRange { start: 'a', end: 'b' };
    let range2 = ClassUnicodeRange { start: 'x', end: 'y' };
    let interval_set = IntervalSet::new(vec![range1, range2]);
    let class_unicode = ClassUnicode { set: interval_set };

    let ranges = class_unicode.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassUnicodeRange { start: 'a', end: 'b' });
    assert_eq!(ranges[1], ClassUnicodeRange { start: 'x', end: 'y' });
}

#[test]
fn test_ranges_with_empty_set() {
    let interval_set = IntervalSet::new(vec![]);
    let class_unicode = ClassUnicode { set: interval_set };

    let ranges = class_unicode.ranges();
    assert_eq!(ranges.len(), 0);
}

#[test]
fn test_ranges_with_single_range() {
    let range = ClassUnicodeRange { start: 'c', end: 'c' };
    let interval_set = IntervalSet::new(vec![range]);
    let class_unicode = ClassUnicode { set: interval_set };

    let ranges = class_unicode.ranges();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassUnicodeRange { start: 'c', end: 'c' });
}

