// Answer 0

#[test]
fn test_peek_error_syntax() {
    struct MockReader {
        position: (usize, usize),
    }

    impl MockReader {
        fn new(line: usize, column: usize) -> Self {
            Self {
                position: (line, column),
            }
        }
        
        fn peek_position(&self) -> (usize, usize) {
            self.position
        }
    }

    struct Error {
        kind: String,
        line: usize,
        column: usize,
    }

    impl Error {
        fn syntax(reason: &str, line: usize, column: usize) -> Self {
            Self {
                kind: format!("Syntax error: {} at line {}, column {}", reason, line, column),
                line,
                column,
            }
        }
    }

    fn peek_error(reader: &MockReader, reason: &str) -> Error {
        let position = reader.peek_position();
        Error::syntax(reason, position.0, position.1)
    }

    let reader = MockReader::new(1, 10);
    let error = peek_error(&reader, "Unexpected token");

    assert_eq!(error.kind, "Syntax error: Unexpected token at line 1, column 10");
    assert_eq!(error.line, 1);
    assert_eq!(error.column, 10);
}

#[test]
#[should_panic]
fn test_peek_error_invalid_position() {
    struct MockReader {
        position: (usize, usize),
    }

    impl MockReader {
        fn new(line: usize, column: usize) -> Self {
            Self {
                position: (line, column),
            }
        }

        fn peek_position(&self) -> (usize, usize) {
            self.position
        }
    }

    struct Error {
        kind: String,
        line: usize,
        column: usize,
    }

    impl Error {
        fn syntax(reason: &str, line: usize, column: usize) -> Self {
            Self {
                kind: format!("Syntax error: {} at line {}, column {}", reason, line, column),
                line,
                column,
            }
        }
    }

    fn peek_error(reader: &MockReader, reason: &str) -> Error {
        let position = reader.peek_position();
        if position.0 == 0 && position.1 == 0 { // Panic condition
            panic!("Position cannot be zero for line and column.");
        }
        Error::syntax(reason, position.0, position.1)
    }

    let reader = MockReader::new(0, 0); // This will trigger a panic
    let _error = peek_error(&reader, "Invalid input");
}

