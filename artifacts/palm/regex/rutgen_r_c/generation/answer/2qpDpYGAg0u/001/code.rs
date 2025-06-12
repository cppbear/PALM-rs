// Answer 0

#[test]
fn test_lower_function_with_valid_start() {
    let range = ClassBytesRange { start: 5, end: 10 };
    assert_eq!(range.lower(), 5);
}

#[test]
fn test_lower_function_with_zero_start() {
    let range = ClassBytesRange { start: 0, end: 10 };
    assert_eq!(range.lower(), 0);
}

#[test]
fn test_lower_function_with_equal_start_end() {
    let range = ClassBytesRange { start: 10, end: 10 };
    assert_eq!(range.lower(), 10);
}

#[test]
fn test_lower_function_with_max_start() {
    let range = ClassBytesRange { start: u8::MAX, end: u8::MAX };
    assert_eq!(range.lower(), u8::MAX);
}

