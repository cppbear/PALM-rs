// Answer 0

#[test]
fn test_case_fold_simple_empty_class_bytes() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_all_ascii_range_a_z() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: b'a', end: b'z' }]);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_all_ascii_range_A_Z() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: b'A', end: b'Z' }]);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_combined_ranges() {
    let mut class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: b'a', end: b'z' },
        ClassBytesRange { start: b'A', end: b'Z' }
    ]);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_single_interval() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: b'c', end: b'c' }]);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_full_ascii_range() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 255 }]);
    class_bytes.case_fold_simple();
}

#[test]
fn test_case_fold_simple_reverse_order_range() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: b'z', end: b'a' }]);
    class_bytes.case_fold_simple();
}

#[test]
#[should_panic]
fn test_case_fold_simple_invalid_range() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 256, end: 255 }]);
    class_bytes.case_fold_simple();
}

