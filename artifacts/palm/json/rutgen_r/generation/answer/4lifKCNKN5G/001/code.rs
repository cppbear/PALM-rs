// Answer 0

#[test]
fn test_line_with_valid_line_number() {
    struct Error {
        line: usize,
    }

    struct MyError {
        err: Error,
    }

    impl MyError {
        pub fn line(&self) -> usize {
            self.err.line
        }
    }

    let error = MyError { err: Error { line: 5 } };
    assert_eq!(error.line(), 5);
}

#[test]
fn test_line_with_first_line() {
    struct Error {
        line: usize,
    }

    struct MyError {
        err: Error,
    }

    impl MyError {
        pub fn line(&self) -> usize {
            self.err.line
        }
    }

    let error = MyError { err: Error { line: 1 } };
    assert_eq!(error.line(), 1);
}

#[test]
fn test_line_with_large_line_number() {
    struct Error {
        line: usize,
    }

    struct MyError {
        err: Error,
    }

    impl MyError {
        pub fn line(&self) -> usize {
            self.err.line
        }
    }

    let error = MyError { err: Error { line: usize::MAX } };
    assert_eq!(error.line(), usize::MAX);
}

