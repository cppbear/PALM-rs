// Answer 0

#[test]
fn test_case_fold_simple_with_full_range() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(65, 122);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_with_partial_lower() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(97, 100);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_with_partial_upper() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(65, 68);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_with_adjacent_lower() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(122, 125);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_with_adjacent_upper() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(90, 92);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_with_contiguous_lower() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(98, 100);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_with_contiguous_upper() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(67, 69);
    range.case_fold_simple(&mut ranges);
}

