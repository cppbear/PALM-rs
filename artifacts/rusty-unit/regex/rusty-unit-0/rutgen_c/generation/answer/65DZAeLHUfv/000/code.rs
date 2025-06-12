// Answer 0

#[test]
fn test_case_fold_simple_with_lowercase() {
    let mut class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: b'a', end: b'z' }
    ]);
    class_bytes.case_fold_simple();
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange { start: b'a', end: b'z' });
    assert_eq!(ranges[1], ClassBytesRange { start: b'A', end: b'Z' });
}

#[test]
fn test_case_fold_simple_with_uppercase() {
    let mut class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: b'A', end: b'Z' }
    ]);
    class_bytes.case_fold_simple();
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange { start: b'a', end: b'z' });
    assert_eq!(ranges[1], ClassBytesRange { start: b'A', end: b'Z' });
}

#[test]
fn test_case_fold_simple_with_mixed() {
    let mut class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: b'a', end: b'a' },
        ClassBytesRange { start: b'A', end: b'A' },
    ]);
    class_bytes.case_fold_simple();
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange { start: b'a', end: b'a' });
    assert_eq!(ranges[1], ClassBytesRange { start: b'A', end: b'A' });
}

#[test]
fn test_case_fold_simple_empty_class() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.case_fold_simple();
    assert!(class_bytes.ranges().is_empty());
}

