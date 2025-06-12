// Answer 0

#[test]
fn test_new_byte_range_valid_case() {
    let range = regex_syntax::new(10, 20);
    assert_eq!(range.start, 10);
    assert_eq!(range.end, 20);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_new_byte_range_start_greater_than_end() {
    // This should panic because start should not be greater than end
    let _range = regex_syntax::new(20, 10);
}

#[test]
fn test_new_byte_range_equal_start_end() {
    let range = regex_syntax::new(15, 15);
    assert_eq!(range.start, 15);
    assert_eq!(range.end, 15);
}

#[test]
fn test_new_byte_range_min_cases() {
    let range = regex_syntax::new(0, 0);
    assert_eq!(range.start, 0);
    assert_eq!(range.end, 0);
}

#[test]
fn test_new_byte_range_max_cases() {
    let range = regex_syntax::new(255, 255);
    assert_eq!(range.start, 255);
    assert_eq!(range.end, 255);
}

