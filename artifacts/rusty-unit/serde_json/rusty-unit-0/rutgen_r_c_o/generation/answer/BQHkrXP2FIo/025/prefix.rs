// Answer 0

#[test]
fn test_classify_category_data() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }
    
    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let error_code = ErrorCode::Message("example message".to_string());
    let error_impl = TestErrorImpl {
        code: error_code,
        line: 1,
        column: 1,
    };
    let error = TestError {
        err: Box::new(error_impl),
    };

    let _ = error.classify();
}

#[test]
fn test_classify_category_data_another_message() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let error_code = ErrorCode::Message("another example".to_string());
    let error_impl = TestErrorImpl {
        code: error_code,
        line: 2,
        column: 2,
    };
    let error = TestError {
        err: Box::new(error_impl),
    };

    let _ = error.classify();
}

#[test]
fn test_classify_category_data_empty_message() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    let error_code = ErrorCode::Message("".to_string());
    let error_impl = TestErrorImpl {
        code: error_code,
        line: 3,
        column: 3,
    };
    let error = TestError {
        err: Box::new(error_impl),
    };

    let _ = error.classify();
}

