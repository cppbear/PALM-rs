// Answer 0

#[test]
fn test_classify_syntax_error() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    enum ErrorCode {
        ExpectedObjectCommaOrEnd,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::ExpectedObjectCommaOrEnd,
        line: 1,
        column: 10,
    };
    
    let error = Error {
        err: Box::new(error_impl),
    };

    let category = error.classify();
    assert_eq!(category, Category::Syntax);
}

