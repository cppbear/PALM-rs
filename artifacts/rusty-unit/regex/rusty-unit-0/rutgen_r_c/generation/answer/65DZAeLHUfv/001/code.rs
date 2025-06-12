// Answer 0

#[test]
fn test_case_fold_simple_empty_class() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.case_fold_simple();
    assert_eq!(class_bytes.ranges(), &[]);
}

#[test]
fn test_case_fold_simple_ascii_lowercase() {
    let ranges = vec![ClassBytesRange { start: b'a', end: b'z' }];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
    assert_eq!(class_bytes.ranges(), &[
        ClassBytesRange { start: b'a', end: b'z' },
        ClassBytesRange { start: b'A', end: b'Z' }
    ]);
}

#[test]
fn test_case_fold_simple_non_overlap() {
    let ranges = vec![
        ClassBytesRange { start: b'0', end: b'9' },
        ClassBytesRange { start: b'a', end: b'z' }
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
    assert_eq!(class_bytes.ranges(), &[
        ClassBytesRange { start: b'0', end: b'9' },
        ClassBytesRange { start: b'a', end: b'z' },
        ClassBytesRange { start: b'A', end: b'Z' }
    ]);
}

#[test]
fn test_case_fold_simple_multiple_ranges() {
    let ranges = vec![
        ClassBytesRange { start: b'a', end: b'c' },
        ClassBytesRange { start: b'x', end: b'y' },
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
    assert_eq!(class_bytes.ranges(), &[
        ClassBytesRange { start: b'a', end: b'c' },
        ClassBytesRange { start: b'A', end: b'C' },
        ClassBytesRange { start: b'x', end: b'y' },
        ClassBytesRange { start: b'X', end: b'Y' },
    ]);
}

#[test]
fn test_case_fold_simple_no_ascii_lowercase() {
    let ranges = vec![ClassBytesRange { start: b'0', end: b'1' }];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.case_fold_simple();
    assert_eq!(class_bytes.ranges(), &[
        ClassBytesRange { start: b'0', end: b'1' }
    ]);
}

