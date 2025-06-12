// Answer 0

#[test]
fn test_peek_invalid_type_string() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct MockDeserializer {
        scratch: Vec<u8>,
        read: MockReader,
    }

    struct MockReader;

    impl MockReader {
        fn parse_str(&self, scratch: &mut Vec<u8>) -> Result<&str, ()> {
            scratch.extend_from_slice(b"test string");
            Ok("test string")
        }
    }

    impl MockDeserializer {
        fn peek_or_null(&self) -> Option<u8> {
            Some(b'"')
        }
        
        fn eat_char(&mut self) {}

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::from("Expected some value")
        }
        
        fn invalid_type(&self, _unexpected: Unexpected, _exp: &dyn Expected) -> Error {
            Error::from("Invalid type")
        }
    }

    let mut deserializer = MockDeserializer {
        scratch: vec![],
        read: MockReader,
    };

    let error = deserializer.peek_invalid_type(&MockExpected {});
    assert!(matches!(error, Error::InvalidType(_, _)));
    assert!(deserializer.scratch == b"test string");
}

