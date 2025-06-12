// Answer 0

#[test]
fn test_is_eof_with_eof_error() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::EofWhileParsingList |
                ErrorCode::EofWhileParsingObject |
                ErrorCode::EofWhileParsingString |
                ErrorCode::EofWhileParsingValue => Category::Eof,
                _ => Category::Data, // default to Data for simplicity
            }
        }

        fn is_eof(&self) -> bool {
            self.classify() == Category::Eof
        }
    }

    enum ErrorCode {
        EofWhileParsingList,
        EofWhileParsingObject,
        EofWhileParsingString,
        EofWhileParsingValue,
        Message(String),
    }

    let eof_error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingList,
            line: 0,
            column: 0,
        }),
    };

    assert!(eof_error.is_eof());
}

#[test]
fn test_is_eof_with_non_eof_error() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::EofWhileParsingList |
                ErrorCode::EofWhileParsingObject |
                ErrorCode::EofWhileParsingString |
                ErrorCode::EofWhileParsingValue => Category::Eof,
                _ => Category::Data, // default to Data for simplicity
            }
        }

        fn is_eof(&self) -> bool {
            self.classify() == Category::Eof
        }
    }

    enum ErrorCode {
        Message(String),
    }

    let data_error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message("Invalid data".to_string()),
            line: 0,
            column: 0,
        }),
    };

    assert!(!data_error.is_eof());
}

