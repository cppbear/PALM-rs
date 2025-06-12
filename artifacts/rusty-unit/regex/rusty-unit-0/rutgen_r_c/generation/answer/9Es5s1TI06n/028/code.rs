// Answer 0

#[test]
fn test_error_kind_class_range_invalid() {
    use std::fmt::Write; // To use the write! macro in tests

    let error_kind = ErrorKind::ClassRangeInvalid;
    let mut output = String::new();
    
    // Call the fmt function
    let result = error_kind.fmt(&mut output);

    // Assert that the result is Ok
    assert!(result.is_ok());
    // Assert that output contains expected error message
    assert_eq!(output, "invalid character class range, the start must be <= the end");
}

#[test]
fn test_error_kind_class_escape_invalid() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassEscapeInvalid;
    let mut output = String::new();
    
    // Call the fmt function
    let result = error_kind.fmt(&mut output);

    // Assert that the result is Ok
    assert!(result.is_ok());
    // Assert that output contains expected error message
    assert_eq!(output, "invalid escape sequence found in character class");
}

