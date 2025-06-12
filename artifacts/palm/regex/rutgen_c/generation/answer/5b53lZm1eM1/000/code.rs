// Answer 0

#[test]
fn test_contains_simple_case_mapping_valid_range() {
    let start = 'a';
    let end = 'z';
    assert!(contains_simple_case_mapping(start, end));
}

#[test]
fn test_contains_simple_case_mapping_empty_range() {
    let start = 'A';
    let end = 'A';
    assert!(contains_simple_case_mapping(start, end));
}

#[test]
#[should_panic]
fn test_contains_simple_case_mapping_invalid_range() {
    let start = 'z';
    let end = 'a';
    contains_simple_case_mapping(start, end);
}

#[test]
fn test_contains_simple_case_mapping_range_with_no_mapping() {
    let start = '\u{007F}'; // DEL character
    let end = '\u{00A0}'; // non-space character
    assert!(!contains_simple_case_mapping(start, end));
}

#[test]
fn test_contains_simple_case_mapping_wide_range() {
    let start = '\u{0041}'; // 'A'
    let end = '\u{007A}'; // 'z'
    assert!(contains_simple_case_mapping(start, end));
}

