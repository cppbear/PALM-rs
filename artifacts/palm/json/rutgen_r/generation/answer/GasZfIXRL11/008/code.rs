// Answer 0

fn test_peek_invalid_type_valid_string() {
    struct MockExpected;
    
    impl Expected for MockExpected {}

    struct MockDeserializer {
        scratch: Vec<u8>,
        read: MockReader,
    }

    struct MockReader;

    impl MockReader {
        fn parse_str(&self, scratch: &mut Vec<u8>) -> Result<String, Error> {
            scratch.extend_from_slice(b"test");
            Ok(String::from("test"))
        }
    }

    impl MockDeserializer {
        fn peek_or_null(&mut self) -> Option<u8> {
            Some(b'"')
        }

        fn eat_char(&mut self) {}
        
        fn fix_position(&mut self, err: Error) -> Error {
            err
        }

        fn peek_error(&mut self, _code: ErrorCode) -> Error {
            Error::custom("Expected some value")
        }
    }

    let mut deserializer = MockDeserializer {
        scratch: Vec::new(),
        read: MockReader,
    };

    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert!(matches!(result, de::Error::invalid_type(Unexpected::Str(_), _)));
}

fn test_peek_invalid_type_reference_string() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct MockDeserializer {
        scratch: Vec<u8>,
        read: MockReader,
    }

    struct MockReader;

    impl MockReader {
        fn parse_str(&self, scratch: &mut Vec<u8>) -> Result<String, Error> {
            scratch.extend_from_slice(b"hello");
            Ok(String::from("hello"))
        }
    }

    impl MockDeserializer {
        fn peek_or_null(&mut self) -> Option<u8> {
            Some(b'"')
        }

        fn eat_char(&mut self) {}

        fn fix_position(&mut self, err: Error) -> Error {
            err
        }

        fn peek_error(&mut self, _code: ErrorCode) -> Error {
            Error::custom("Expected some value")
        }
    }

    let mut deserializer = MockDeserializer {
        scratch: Vec::new(),
        read: MockReader,
    };

    let exp = MockExpected;
    let result = deserializer.peek_invalid_type(&exp);
    assert!(matches!(result, de::Error::invalid_type(Unexpected::Str(_), _)));
}

