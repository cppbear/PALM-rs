// Answer 0

#[test]
fn test_classify_syntax_invalid_escape() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }
    
    #[derive(Debug)]
    enum ErrorCode {
        InvalidEscape,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::InvalidEscape => Category::Syntax,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    enum Category {
        Syntax,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::InvalidEscape,
        line: 1,
        column: 5,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Syntax);
}

