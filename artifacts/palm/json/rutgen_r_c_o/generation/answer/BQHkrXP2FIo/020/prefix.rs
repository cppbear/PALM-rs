// Answer 0

#[test]
fn test_classify_eof_while_parsing_value() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingValue,
        line: 1,
        column: 1,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.classify();
}

#[test]
fn test_classify_eof_while_parsing_object() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingObject,
        line: 2,
        column: 5,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.classify();
}

#[test]
fn test_classify_eof_while_parsing_string() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingString,
        line: 3,
        column: 10,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.classify();
}

#[test]
fn test_classify_eof_while_parsing_list() {
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingList,
        line: 4,
        column: 15,
    };
    let error = Error {
        err: Box::new(error_impl),
    };
    error.classify();
}

