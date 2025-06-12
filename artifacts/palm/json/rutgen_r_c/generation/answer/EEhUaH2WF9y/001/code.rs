// Answer 0

#[test]
fn test_is_data_with_data_error() {
    struct TestError {
        code: ErrorCode,
    }
    
    let error_impl = ErrorImpl {
        code: ErrorCode::Message(String::from("semantic error")),
        line: 1,
        column: 5,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert!(error.is_data());
}

#[test]
fn test_is_data_with_io_error() {
    struct TestError {
        code: ErrorCode,
    }
    
    let error_impl = ErrorImpl {
        code: ErrorCode::Io(io::Error::new(io::ErrorKind::Other, "I/O error")),
        line: 2,
        column: 10,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert!(!error.is_data());
}

#[test]
fn test_is_data_with_syntax_error() {
    struct TestError {
        code: ErrorCode,
    }
    
    let error_impl = ErrorImpl {
        code: ErrorCode::ExpectedDoubleQuote,
        line: 3,
        column: 15,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert!(!error.is_data());
}

#[test]
fn test_is_data_with_eof_error() {
    struct TestError {
        code: ErrorCode,
    }
    
    let error_impl = ErrorImpl {
        code: ErrorCode::EofWhileParsingValue,
        line: 4,
        column: 20,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert!(!error.is_data());
}

#[test]
fn test_is_data_with_other_semantic_error() {
    struct TestError {
        code: ErrorCode,
    }
    
    let error_impl = ErrorImpl {
        code: ErrorCode::Message(String::from("another semantic issue")),
        line: 5,
        column: 25,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };
    
    assert!(error.is_data());
}

