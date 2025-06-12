// Answer 0

#[test]
fn test_error_creates_syntax_error_with_correct_position() {
    struct MockReader {
        position: Position,
    }

    impl MockReader {
        fn new(line: usize, column: usize) -> Self {
            Self {
                position: Position { line, column },
            }
        }

        fn read(&self) -> &Self {
            self
        }

        fn position(&self) -> &Position {
            &self.position
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    struct Error {
        reason: ErrorCode,
        line: usize,
        column: usize,
    }

    impl Error {
        fn syntax(reason: ErrorCode, line: usize, column: usize) -> Self {
            Self { reason, line, column }
        }
    }

    struct ErrorCode;

    impl MockReader {
        fn error(&self, reason: ErrorCode) -> Error {
            let position = self.read().position();
            Error::syntax(reason, position.line, position.column)
        }
    }

    let reader = MockReader::new(2, 5);
    let error_code = ErrorCode;
    let error = reader.error(error_code);

    assert_eq!(error.line, 2);
    assert_eq!(error.column, 5);
}

