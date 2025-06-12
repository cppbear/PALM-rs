// Answer 0

#[test]
fn test_case_fold_simple_lowercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'a', b'z');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].start(), b'A');
    assert_eq!(ranges[0].end(), b'Z');
}

#[test]
fn test_case_fold_simple_uppercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'A', b'Z');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].start(), b'a');
    assert_eq!(ranges[0].end(), b'z');
}

#[test]
fn test_case_fold_simple_mixed_case() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'm', b'P');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0].start(), b'A');
    assert_eq!(ranges[0].end(), b'Z');
    assert_eq!(ranges[1].start(), b'm');
    assert_eq!(ranges[1].end(), b'p');
}

#[test]
fn test_case_fold_simple_no_case() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'1', b'5');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 0);
}

#[test]
fn test_case_fold_simple_partial_overlap() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'g', b'R');
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0].start(), b'A');
    assert_eq!(ranges[0].end(), b'Z');
    assert_eq!(ranges[1].start(), b'g');
    assert_eq!(ranges[1].end(), b'r');
}

