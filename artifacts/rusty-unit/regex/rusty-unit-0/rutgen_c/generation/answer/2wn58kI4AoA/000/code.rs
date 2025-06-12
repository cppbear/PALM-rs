// Answer 0

#[test]
fn test_iter_empty() {
    let class_unicode = ClassUnicode::empty();
    let mut iter = class_unicode.iter();
    assert_eq!(iter.0.0.len(), 0); // Expect the iterator to be empty
}

#[test]
fn test_iter_single_range() {
    let range = ClassUnicodeRange { start: 'a', end: 'b' };
    let class_unicode = ClassUnicode::new(vec![range]);
    let mut iter = class_unicode.iter();
    let mut ranges: Vec<ClassUnicodeRange> = iter.0.0.cloned().collect();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], range);
}

#[test]
fn test_iter_multiple_ranges() {
    let range1 = ClassUnicodeRange { start: 'a', end: 'b' };
    let range2 = ClassUnicodeRange { start: 'd', end: 'f' };
    let class_unicode = ClassUnicode::new(vec![range1, range2]);
    let mut iter = class_unicode.iter();
    let mut ranges: Vec<ClassUnicodeRange> = iter.0.0.cloned().collect();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], range1);
    assert_eq!(ranges[1], range2);
}

