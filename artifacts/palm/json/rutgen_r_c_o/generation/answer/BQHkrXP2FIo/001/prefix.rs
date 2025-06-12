// Answer 0

#[test]
fn test_classify_recursion_limit_exceeded() {
    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    enum ErrorCode {
        RecursionLimitExceeded,
    }

    let error_impl = ErrorImpl {
        code: ErrorCode::RecursionLimitExceeded,
        line: 1,
        column: 1,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    let _category = error.classify();
}

