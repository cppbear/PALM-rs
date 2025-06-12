// Answer 0

#[test]
fn test_error_kind_display_group_name_empty() {
    use std::fmt::Write;

    let error = ErrorKind::GroupNameEmpty;
    let mut output = String::new();

    // Call the fmt function
    let result = write!(output, "{}", error);

    // Check if the result is Ok (no error occurred while writing)
    assert!(result.is_ok());
    // Check if the output is as expected
    assert_eq!(output, "empty capture group name");
}

#[test]
fn test_error_kind_display_group_name_duplicate() {
    use std::fmt::Write;

    let error = ErrorKind::GroupNameDuplicate {
        original: Span {
            start: Position(0),
            end: Position(1),
        },
    };
    let mut output = String::new();

    // Call the fmt function
    let result = write!(output, "{}", error);

    // Check if the result is Ok (no error occurred while writing)
    assert!(result.is_ok());
    // Check if the output is as expected
    assert_eq!(output, "duplicate capture group name");
}

