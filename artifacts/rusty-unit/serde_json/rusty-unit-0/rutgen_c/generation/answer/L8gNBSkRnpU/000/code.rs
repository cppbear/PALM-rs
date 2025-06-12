// Answer 0

#[test]
fn test_fix_position_no_line() {
    let error_code = ErrorCode::ExpectedColon;
    let error = Error::syntax(error_code, 0, 0);
    
    let new_error = error.fix_position(|code| Error::syntax(code, 1, 1));
    
    assert!(matches!(new_error.err.code, ErrorCode::ExpectedColon));
}

#[test]
fn test_fix_position_with_line() {
    let error_code = ErrorCode::ExpectedValue;
    let error = Error::syntax(error_code, 1, 1);
    
    let new_error = error.fix_position(|code| Error::syntax(code, 2, 2));

    assert_eq!(new_error.err.line, 1);
    assert_eq!(new_error.err.column, 1);
    assert!(matches!(new_error.err.code, ErrorCode::ExpectedValue));
}

