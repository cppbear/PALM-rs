// Answer 0

#[test]
fn test_classify_eof_while_parsing_value() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn new_eof_while_parsing_value() -> Self {
            let error_impl = ErrorImpl {
                code: ErrorCode::EofWhileParsingValue,
                line: 1,
                column: 5,
            };
            Error {
                err: Box::new(error_impl),
            }
        }
    }

    let error = TestError::new_eof_while_parsing_value();
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_object() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn new_eof_while_parsing_object() -> Self {
            let error_impl = ErrorImpl {
                code: ErrorCode::EofWhileParsingObject,
                line: 2,
                column: 3,
            };
            Error {
                err: Box::new(error_impl),
            }
        }
    }

    let error = TestError::new_eof_while_parsing_object();
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_string() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn new_eof_while_parsing_string() -> Self {
            let error_impl = ErrorImpl {
                code: ErrorCode::EofWhileParsingString,
                line: 3,
                column: 4,
            };
            Error {
                err: Box::new(error_impl),
            }
        }
    }

    let error = TestError::new_eof_while_parsing_string();
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_list() {
    struct TestError {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn new_eof_while_parsing_list() -> Self {
            let error_impl = ErrorImpl {
                code: ErrorCode::EofWhileParsingList,
                line: 4,
                column: 6,
            };
            Error {
                err: Box::new(error_impl),
            }
        }
    }

    let error = TestError::new_eof_while_parsing_list();
    assert_eq!(error.classify(), Category::Eof);
}

