// Answer 0

#[test]
fn test_is_syntax_for_syntax_error() {
    struct MyErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }
    
    let error_impl = MyErrorImpl {
        code: ErrorCode::InvalidNumber,
        line: 1,
        column: 10,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    assert!(error.is_syntax());
}

#[test]
fn test_is_syntax_for_io_error() {
    struct MyErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    let error_impl = MyErrorImpl {
        code: ErrorCode::Io("Some I/O error".to_string()),
        line: 2,
        column: 5,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    assert!(!error.is_syntax());
}

#[test]
fn test_is_syntax_for_data_error() {
    struct MyErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    let error_impl = MyErrorImpl {
        code: ErrorCode::Message("Data error".to_string()),
        line: 3,
        column: 15,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    assert!(!error.is_syntax());
}

#[test]
fn test_is_syntax_for_eof_error() {
    struct MyErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    let error_impl = MyErrorImpl {
        code: ErrorCode::EofWhileParsingValue,
        line: 4,
        column: 20,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    assert!(!error.is_syntax());
}

