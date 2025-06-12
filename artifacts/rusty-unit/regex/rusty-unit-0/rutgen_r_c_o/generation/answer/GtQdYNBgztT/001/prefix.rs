// Answer 0

#[test]
fn test_case_fold_simple_no_lowercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(0, 127);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_no_uppercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(0, 127);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_all_non_ascii() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(128, 255);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_partial_ascii() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(10, 20);
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_fully_inside_lowercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'a', b'z');
    range.case_fold_simple(&mut ranges);
}

#[test]
fn test_case_fold_simple_fully_inside_uppercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'A', b'Z');
    range.case_fold_simple(&mut ranges);
}

