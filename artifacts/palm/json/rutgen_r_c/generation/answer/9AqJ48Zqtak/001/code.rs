// Answer 0

#[test]
fn test_error_display_empty() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    impl Display for TestErrorImpl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error code: {:?}, Line: {}, Column: {}", self.code, self.line, self.column)
        }
    }
    
    let error_impl = TestErrorImpl {
        code: ErrorCode::unknown(),
        line: 0,
        column: 0,
    };
    let error = TestError {
        err: Box::new(error_impl),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Error code: Unknown, Line: 0, Column: 0");
}

#[test]
fn test_error_display_with_values() {
    struct TestErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct TestError {
        err: Box<TestErrorImpl>,
    }

    impl Display for TestErrorImpl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error code: {:?}, Line: {}, Column: {}", self.code, self.line, self.column)
        }
    }

    let error_impl = TestErrorImpl {
        code: ErrorCode::InvalidData,
        line: 12,
        column: 5,
    };
    let error = TestError {
        err: Box::new(error_impl),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Error code: InvalidData, Line: 12, Column: 5");
}

