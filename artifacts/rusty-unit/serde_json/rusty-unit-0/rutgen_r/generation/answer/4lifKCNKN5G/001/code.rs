// Answer 0

#[test]
fn test_line_with_valid_line_number() {
    struct MockError {
        line: usize,
    }

    struct ErrorWrapper {
        err: MockError,
    }

    let error = ErrorWrapper { err: MockError { line: 10 } };
    assert_eq!(error.line(), 10);
}

#[test]
fn test_line_with_zero_line_number() {
    struct MockError {
        line: usize,
    }

    struct ErrorWrapper {
        err: MockError,
    }

    let error = ErrorWrapper { err: MockError { line: 0 } };
    assert_eq!(error.line(), 0);
}

#[test]
fn test_line_with_large_line_number() {
    struct MockError {
        line: usize,
    }

    struct ErrorWrapper {
        err: MockError,
    }

    let error = ErrorWrapper { err: MockError { line: usize::MAX } };
    assert_eq!(error.line(), usize::MAX);
}

#[test]
#[should_panic]
fn test_line_with_uninitialized_line_number() {
    struct MockError {
        line: Option<usize>,
    }

    struct ErrorWrapper {
        err: MockError,
    }

    impl ErrorWrapper {
        pub fn line(&self) -> usize {
            self.err.line.expect("Line number not initialized")
        }
    }

    let error = ErrorWrapper { err: MockError { line: None } };
    error.line();
}

