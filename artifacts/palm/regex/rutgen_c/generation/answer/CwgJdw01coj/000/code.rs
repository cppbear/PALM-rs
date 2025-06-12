// Answer 0

#[test]
fn test_case_fold_simple_with_empty_class() {
    let mut class_unicode = ClassUnicode::empty();
    class_unicode.case_fold_simple();
    assert_eq!(class_unicode.ranges().len(), 0);
}

#[test]
fn test_case_fold_simple_with_single_range() {
    let mut class_unicode = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    class_unicode.case_fold_simple();
    let ranges = class_unicode.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassUnicodeRange { start: 'a', end: 'z' });
    assert_eq!(ranges[1], ClassUnicodeRange { start: 'A', end: 'Z' });
}

#[test]
fn test_case_fold_simple_with_no_case_variation() {
    let mut class_unicode = ClassUnicode::new(vec![ClassUnicodeRange { start: '0', end: '9' }]);
    class_unicode.case_fold_simple();
    let ranges = class_unicode.ranges();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassUnicodeRange { start: '0', end: '9' });
}

#[test]
fn test_case_fold_simple_with_mixed_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: '0', end: '9' },
    ]);
    class_unicode.case_fold_simple();
    let ranges = class_unicode.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassUnicodeRange { start: 'a', end: 'z' });
    assert_eq!(ranges[1], ClassUnicodeRange { start: 'A', end: 'Z' });
}

