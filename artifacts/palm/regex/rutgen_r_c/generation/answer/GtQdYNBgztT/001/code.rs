// Answer 0

#[test]
fn test_case_fold_simple_no_intersection() {
    let mut ranges = Vec::new();
    let class_bytes_range = ClassBytesRange::new(b'0', b'9'); // No intersection with a-z or A-Z
    class_bytes_range.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty());
}

#[test]
fn test_case_fold_simple_partial_intersection_lowercase() {
    let mut ranges = Vec::new();
    let class_bytes_range = ClassBytesRange::new(b'a', b'b'); // Intersects with a-z, but not A-Z
    class_bytes_range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].start(), b'A' - 32);
    assert_eq!(ranges[0].end(), b'B' - 32);
}

#[test]
fn test_case_fold_simple_partial_intersection_uppercase() {
    let mut ranges = Vec::new();
    let class_bytes_range = ClassBytesRange::new(b'X', b'Y'); // Intersects with A-Z, but not a-z
    class_bytes_range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].start(), b'x' + 32);
    assert_eq!(ranges[0].end(), b'y' + 32);
}

#[test]
fn test_case_fold_simple_full_intersection() {
    let mut ranges = Vec::new();
    let class_bytes_range = ClassBytesRange::new(b'a', b'z'); // Fully intersects with a-z
    class_bytes_range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].start(), b'A' - 32);
    assert_eq!(ranges[0].end(), b'Z' - 32);
}

#[test]
fn test_case_fold_simple_empty_range() {
    let mut ranges = Vec::new();
    let class_bytes_range = ClassBytesRange::new(b'a', b'a'); // No range
    class_bytes_range.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty());
}

