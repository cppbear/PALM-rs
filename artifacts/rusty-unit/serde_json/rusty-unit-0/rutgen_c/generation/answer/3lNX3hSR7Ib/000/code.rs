// Answer 0

#[test]
fn test_peek_end_of_value_with_valid_characters() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl read::Read<'_> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn peek_position(&self) -> Position {
            Position {
                line: 1,
                column: self.position,
            }
        }
    }

    let mock_data = vec![b' ', b'\n', b'\t', b'{', b'}', b':'];
    let mut deserializer = StreamDeserializer::new(MockRead {
        data: mock_data,
        position: 0,
    });

    let result = deserializer.peek_end_of_value();
    assert!(result.is_ok());
}

#[test]
fn test_peek_end_of_value_with_invalid_character() {
    struct MockReadInvalid {
        data: Vec<u8>,
        position: usize,
    }

    impl read::Read<'_> for MockReadInvalid {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn peek_position(&self) -> Position {
            Position {
                line: 1,
                column: self.position,
            }
        }
    }

    let mock_data = vec![b'x']; // Invalid character
    let mut deserializer = StreamDeserializer::new(MockReadInvalid {
        data: mock_data,
        position: 0,
    });

    let result = deserializer.peek_end_of_value();
    assert!(result.is_err());
    if let Err(error) = result {
        assert!(matches!(error.err.code, ErrorCode::TrailingCharacters));
    }
}

#[test]
fn test_peek_end_of_value_with_no_input() {
    struct MockReadEmpty {
        data: Vec<u8>,
    }

    impl read::Read<'_> for MockReadEmpty {
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: 0 }
        }
    }

    let mut deserializer = StreamDeserializer::new(MockReadEmpty { data: vec![] });

    let result = deserializer.peek_end_of_value();
    assert!(result.is_ok());
}

