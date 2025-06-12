// Answer 0

#[test]
fn test_peek_invalid_type_null() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'n'], // Simulating the input 'null'
                index: 0,
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(self.index).copied()
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            // Simulating a successful parse
            Ok(())
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Unit, &exp));
}

#[test]
fn test_peek_invalid_type_empty_array() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'['], // Simulating the input for an empty array
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(0).copied()
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Seq, &exp));
}

#[test]
fn test_peek_invalid_type_empty_string() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
        index: usize,
        scratch: Vec<u8>,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'"'], // Simulating the input for an empty string
                index: 0,
                scratch: vec![],
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(self.index).copied()
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn read_parse_str(&mut self, _: &mut Vec<u8>) -> Result<&[u8], ()> {
            // Simulating successful string parsing
            self.scratch.extend_from_slice(b"example");
            Ok(&self.scratch)
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Str(b"example"), &exp));
}

#[test]
fn test_peek_invalid_type_false() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'f'], // Simulating the input 'false'
                index: 0,
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(self.index).copied()
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            // Simulating a successful parse
            Ok(())
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Bool(false), &exp));
}

#[test]
fn test_peek_invalid_type_empty_object() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'{'], // Simulating the input for an empty object
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(0).copied()
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Map, &exp));
}

#[test]
fn test_peek_invalid_type_true() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b't'], // Simulating the input 'true'
                index: 0,
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(self.index).copied()
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            // Simulating a successful parse
            Ok(())
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Bool(true), &exp));
}

#[test]
fn test_peek_invalid_type_negative_number() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'-'], // Simulating the input for a negative number
                index: 0,
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(self.index).copied()
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _: bool) -> Result<MockNumber, ()> {
            // Simulating successful number parsing
            Ok(MockNumber)
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    struct MockNumber;

    impl MockNumber {
        fn invalid_type(&self, _: &dyn Expected) -> de::Error {
            de::Error::invalid_type(Unexpected::Units, &())
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Units, &exp));
}

#[test]
fn test_peek_invalid_type_positive_number() {
    struct MockExpected;
    struct MockDeserializer {
        buffer: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new() -> Self {
            Self {
                buffer: vec![b'5'], // Simulating the input for a positive number
                index: 0,
            }
        }

        fn peek_or_null(&mut self) -> Option<u8> {
            self.buffer.get(self.index).copied()
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _: bool) -> Result<MockNumber, ()> {
            // Simulating successful number parsing
            Ok(MockNumber)
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    struct MockNumber;

    impl MockNumber {
        fn invalid_type(&self, _: &dyn Expected) -> de::Error {
            de::Error::invalid_type(Unexpected::Units, &())
        }
    }

    let mut deserializer = MockDeserializer::new();
    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Units, &exp));
}

