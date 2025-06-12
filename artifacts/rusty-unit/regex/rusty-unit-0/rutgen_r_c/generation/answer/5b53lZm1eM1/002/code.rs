// Answer 0

#[test]
#[should_panic]
fn test_contains_simple_case_mapping_panic_start_greater_than_end() {
    let start = 'z';
    let end = 'a';
    contains_simple_case_mapping(start, end);
}

#[test]
fn test_contains_simple_case_mapping_empty_range() {
    let start = 'a';
    let end = 'a';
    assert_eq!(contains_simple_case_mapping(start, end), false);
}

#[test]
fn test_contains_simple_case_mapping_regular_case() {
    let start = 'A';
    let end = 'Z';
    assert_eq!(contains_simple_case_mapping(start, end), true);
}

#[test]
fn test_contains_simple_case_mapping_full_range() {
    let start = '\u{0000}';
    let end = '\u{FFFF}';
    assert_eq!(contains_simple_case_mapping(start, end), true);
}

