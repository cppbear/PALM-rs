// Answer 0

#[test]
fn test_fix_position_when_line_is_zero() {
    use std::io;

    struct TestError {
        line: usize,
        column: usize,
    }

    let error_code = ErrorCode::Message(Box::from("Test error".to_string()));
    let error_instance = Error {
        err: Box::new(ErrorImpl {
            code: error_code,
            line: 0,
            column: 0,
        }),
    };

    let modified_error = error_instance.fix_position(|code| {
        Error::syntax(code, 1, 1)
    });

    assert_eq!(modified_error.err.line, 1);
}

#[test]
fn test_fix_position_when_line_is_not_zero() {
    use std::io;

    struct TestError {
        line: usize,
        column: usize,
    }

    let error_code = ErrorCode::Message(Box::from("Test error".to_string()));
    let error_instance = Error {
        err: Box::new(ErrorImpl {
            code: error_code,
            line: 1,
            column: 0,
        }),
    };

    let modified_error = error_instance.fix_position(|code| {
        Error::syntax(code, 1, 1)
    });

    assert_eq!(modified_error.err.line, 1);
    assert!(matches!(modified_error.err.code, ErrorCode::Message(_)));
}

