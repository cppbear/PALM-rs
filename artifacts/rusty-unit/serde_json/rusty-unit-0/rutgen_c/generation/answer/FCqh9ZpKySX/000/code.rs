// Answer 0

#[test]
fn test_make_error_with_valid_message() {
    let message = String::from("Parsing error at line 10 column 5");
    let error = make_error(message);
    assert_eq!(error.err.line, 10);
    assert_eq!(error.err.column, 5);
}

#[test]
fn test_make_error_with_invalid_message_no_line_col() {
    let message = String::from("Parsing error occurred");
    let error = make_error(message);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_make_error_with_invalid_message_invalid_line() {
    let message = String::from("Parsing error at line X column Y");
    let error = make_error(message);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_make_error_with_missing_column() {
    let message = String::from("Parsing error at line 10");
    let error = make_error(message);
    assert_eq!(error.err.line, 10);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_make_error_with_unexpected_suffix() {
    let message = String::from("Parsing error at line 10 column 5 extra characters");
    let error = make_error(message);
    assert_eq!(error.err.line, 10);
    assert_eq!(error.err.column, 5);
}

