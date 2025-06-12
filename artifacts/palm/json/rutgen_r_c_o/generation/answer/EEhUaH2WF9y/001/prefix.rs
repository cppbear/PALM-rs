// Answer 0

#[test]
fn test_is_data_message_error() {
    let error_impl = ErrorImpl { 
        code: ErrorCode::Message("Semantic error".to_string()), 
        line: 1, 
        column: 5 
    };
    let error = Error { err: Box::new(error_impl) };
    error.is_data();
}

#[test]
fn test_is_data_invalid_number_error() {
    let error_impl = ErrorImpl { 
        code: ErrorCode::InvalidNumber, 
        line: 2, 
        column: 3 
    };
    let error = Error { err: Box::new(error_impl) };
    error.is_data();
}

#[test]
fn test_is_data_key_must_be_string_error() {
    let error_impl = ErrorImpl { 
        code: ErrorCode::KeyMustBeAString, 
        line: 3, 
        column: 10 
    };
    let error = Error { err: Box::new(error_impl) };
    error.is_data();
}

#[test]
fn test_is_data_eof_while_parsing_object_error() {
    let error_impl = ErrorImpl { 
        code: ErrorCode::EofWhileParsingObject, 
        line: 4, 
        column: 1 
    };
    let error = Error { err: Box::new(error_impl) };
    error.is_data();
}

#[test]
fn test_is_data_trailing_comma_error() {
    let error_impl = ErrorImpl { 
        code: ErrorCode::TrailingComma, 
        line: 5, 
        column: 8 
    };
    let error = Error { err: Box::new(error_impl) };
    error.is_data();
}

