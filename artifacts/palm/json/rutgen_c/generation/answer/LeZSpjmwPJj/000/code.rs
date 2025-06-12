// Answer 0

#[test]
fn test_error_with_valid_read_position() {
    struct MockReader {
        pos: Position,
    }

    impl MockReader {
        fn new(line: usize, column: usize) -> Self {
            MockReader {
                pos: Position { line, column },
            }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn position(&self) -> Position {
            self.pos.clone()
        }
    }

    let reader = MockReader::new(5, 10);
    let result: Result<()> = error(&reader, ErrorCode::ExpectedDoubleQuote);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.err.line, 5);
        assert_eq!(err.err.column, 10);
    }
}

#[test]
fn test_error_with_zero_position() {
    struct ZeroPositionReader;

    impl<'de> Read<'de> for ZeroPositionReader {
        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }
    }

    let reader = ZeroPositionReader;
    let result: Result<()> = error(&reader, ErrorCode::EofWhileParsingList);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.err.line, 0);
        assert_eq!(err.err.column, 0);
    }
}

