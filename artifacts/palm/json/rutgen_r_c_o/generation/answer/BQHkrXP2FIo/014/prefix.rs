// Answer 0

#[test]
fn test_classify_expected_double_quote() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    let test_error_impl = TestErrorImpl {
        code: ErrorCode::ExpectedDoubleQuote,
        line: 10,
        column: 5,
    };

    let test_error = Error {
        err: Box::new(test_error_impl),
    };

    test_error.classify();
}

