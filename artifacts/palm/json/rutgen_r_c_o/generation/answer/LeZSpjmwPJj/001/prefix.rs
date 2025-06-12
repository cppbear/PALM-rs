// Answer 0

#[test]
fn test_error_with_line_zero_and_column_zero() {
    struct TestReader {
        line: usize,
        column: usize,
    }

    impl TestReader {
        fn new(line: usize, column: usize) -> Self {
            Self { line, column }
        }
    }

    trait Read<'de> {
        fn position(&self) -> Position;
    }

    impl<'de> Read<'de> for TestReader {
        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let reader = TestReader::new(0, 0);
    let reason = ErrorCode::ExpectedColon;
    let _ = error(&reader, reason);
}

#[test]
fn test_error_with_line_at_maximum_and_column_at_maximum() {
    struct TestReader {
        line: usize,
        column: usize,
    }

    impl TestReader {
        fn new(line: usize, column: usize) -> Self {
            Self { line, column }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let reader = TestReader::new(128, 256);
    let reason = ErrorCode::TrailingComma;
    let _ = error(&reader, reason);
}

#[test]
fn test_error_with_middle_line_and_column() {
    struct TestReader {
        line: usize,
        column: usize,
    }

    impl TestReader {
        fn new(line: usize, column: usize) -> Self {
            Self { line, column }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let reader = TestReader::new(64, 128);
    let reason = ErrorCode::InvalidNumber;
    let _ = error(&reader, reason);
}

#[test]
fn test_error_with_non_zero_line_and_column() {
    struct TestReader {
        line: usize,
        column: usize,
    }

    impl TestReader {
        fn new(line: usize, column: usize) -> Self {
            Self { line, column }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn position(&self) -> Position {
            Position {
                line: self.line,
                column: self.column,
            }
        }
    }

    let reader = TestReader::new(1, 1);
    let reason = ErrorCode::EofWhileParsingObject;
    let _ = error(&reader, reason);
}

