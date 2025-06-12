// Answer 0

#[test]
fn test_peek_end_of_value_with_space() {
    struct MockDeserializer {
        input: Vec<u8>,
        position: (usize, usize),
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: (1, 1) }
        }

        fn peek(&mut self) -> Option<u8> {
            self.input.get(0).copied()
        }

        fn read(&mut self) {
            self.input.remove(0);
        }

        fn peek_position(&self) -> (usize, usize) {
            self.position
        }
    }

    let mut deserializer = MockDeserializer::new(b" \n\t\r\"[]{},".to_vec());
    assert!(peek_end_of_value(&mut deserializer).is_ok());
}

#[test]
fn test_peek_end_of_value_with_comma() {
    struct MockDeserializer {
        input: Vec<u8>,
        position: (usize, usize),
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: (1, 1) }
        }

        fn peek(&mut self) -> Option<u8> {
            self.input.get(0).copied()
        }

        fn read(&mut self) {
            self.input.remove(0);
        }

        fn peek_position(&self) -> (usize, usize) {
            self.position
        }
    }

    let mut deserializer = MockDeserializer::new(b"," .to_vec());
    assert!(peek_end_of_value(&mut deserializer).is_ok());
}

#[test]
fn test_peek_end_of_value_with_trailing_character() {
    struct MockDeserializer {
        input: Vec<u8>,
        position: (usize, usize),
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: (1, 1) }
        }

        fn peek(&mut self) -> Option<u8> {
            self.input.get(0).copied()
        }

        fn read(&mut self) {
            self.input.remove(0);
        }

        fn peek_position(&self) -> (usize, usize) {
            self.position
        }
    }

    let mut deserializer = MockDeserializer::new(b"x" .to_vec());
    let result = peek_end_of_value(&mut deserializer);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.code, ErrorCode::TrailingCharacters);
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 1);
    }
}

