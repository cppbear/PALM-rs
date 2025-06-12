// Answer 0

#[test]
fn test_case_fold_simple_inclusive_lowercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'a', b'z');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(0, 25));
}

#[test]
fn test_case_fold_simple_inclusive_uppercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'A', b'Z');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(32, 57));
}

#[test]
fn test_case_fold_simple_mixed_case() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'a', b'F');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange::new(0, 25)); // a-z mapped to A-Z
    assert_eq!(ranges[1], ClassBytesRange::new(32, 57)); // A-F mapped to a-f
}

#[test]
fn test_case_fold_simple_exclusive_lowercase_end() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'a', b'x');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(0, 23)); // a-x maps to A-X
}

#[test]
fn test_case_fold_simple_exclusive_uppercase_start() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'B', b'Z');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(34, 57)); // B-Z maps to b-z
}

