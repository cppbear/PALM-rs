// Answer 0

#[test]
fn test_make_error_with_valid_msg() {
    let msg = String::from("An error occurred at line 5 column 10");
    let error = make_error(msg.clone());
    assert_eq!(error.err.code, ErrorCode::Message(msg.into_boxed_str()));
    assert_eq!(error.err.line, 5);
    assert_eq!(error.err.column, 10);
}

#[test]
fn test_make_error_with_no_line_column() {
    let msg = String::from("An error occurred");
    let error = make_error(msg.clone());
    assert_eq!(error.err.code, ErrorCode::Message(msg.into_boxed_str()));
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_make_error_with_partial_line_info() {
    let msg = String::from("An error occurred at line 5");
    let error = make_error(msg.clone());
    assert_eq!(error.err.code, ErrorCode::Message(msg.into_boxed_str()));
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_make_error_with_invalid_values() {
    let msg = String::from("An error occurred at line abc column 10");
    let error = make_error(msg.clone());
    assert_eq!(error.err.code, ErrorCode::Message(msg.into_boxed_str()));
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

