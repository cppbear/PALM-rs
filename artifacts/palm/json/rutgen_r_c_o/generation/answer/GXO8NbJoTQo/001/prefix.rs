// Answer 0

#[test]
fn test_is_eof_eof_while_parsing_list() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingList,
        line: 0,
        column: 0,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.is_eof();
}

#[test]
fn test_is_eof_eof_while_parsing_object() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingObject,
        line: 5,
        column: 3,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.is_eof();
}

#[test]
fn test_is_eof_eof_while_parsing_string() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingString,
        line: 3,
        column: 1,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.is_eof();
}

#[test]
fn test_is_eof_eof_while_parsing_value() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingValue,
        line: 10,
        column: 10,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.is_eof();
}

#[test]
fn test_is_eof_invalid_error_code() {
    let error_impl = ErrorImpl {
        code: ErrorCode::Message("Some error".to_string()),
        line: 4,
        column: 2,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.is_eof();
}

