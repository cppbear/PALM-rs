// Answer 0

#[test]
fn test_peek_error() {
    struct MockRead {
        position: Position,
    }

    struct Position {
        line: usize,
        column: usize,
    }

    impl MockRead {
        fn peek_position(&self) -> Position {
            self.position.clone()
        }
    }

    struct ErrorCode; // Placeholder for demonstration
    struct Error {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    impl Error {
        fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
            Error { code, line, column }
        }
    }

    let mock_read = MockRead { position: Position { line: 5, column: 10 } };
    let error_code = ErrorCode;

    let error = mock_read.peek_error(error_code.clone());

    assert_eq!(error.line, 5);
    assert_eq!(error.column, 10);
}

