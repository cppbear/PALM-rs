// Answer 0

#[test]
fn test_peek_invalid_type_with_invalid_map() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct MockDeserializer {
        current_char: u8,
    }

    impl MockDeserializer {
        fn peek_or_null(&mut self) -> Option<u8> {
            Some(self.current_char)
        }

        fn eat_char(&mut self) {
            // Simulate eating the current character
        }

        fn parse_ident(&self, _ident: &[u8]) -> Result<(), Error> {
            Err(Error)
        }

        fn parse_any_number(&self, _is_positive: bool) -> Result<Number, Error> {
            Err(Error)
        }

        fn read(&mut self) -> &mut Self {
            self
        }

        fn fix_position(&self, err: Error) -> Error {
            err // Just return the error for the mock
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error
        }
    }

    impl Deserializer for MockDeserializer {
        type Error = Error;

        fn peek_invalid_type(&mut self, exp: &dyn Expected) -> Error {
            let err = match self.peek_or_null().unwrap_or(b'\x00') {
                b'{' => de::Error::invalid_type(Unexpected::Map, exp),
                _ => self.peek_error(ErrorCode::ExpectedSomeValue),
            };

            self.fix_position(err)
        }
    }

    let mut deserializer = MockDeserializer { current_char: b'{' };
    let expected = MockExpected;

    let result = deserializer.peek_invalid_type(&expected);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Map, &expected));
}

#[test]
fn test_peek_invalid_type_with_invalid_seq() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct MockDeserializer {
        current_char: u8,
    }

    impl MockDeserializer {
        fn peek_or_null(&mut self) -> Option<u8> {
            Some(self.current_char)
        }

        fn eat_char(&mut self) {
            // Simulate eating the current character
        }

        fn parse_ident(&self, _ident: &[u8]) -> Result<(), Error> {
            Err(Error)
        }

        fn parse_any_number(&self, _is_positive: bool) -> Result<Number, Error> {
            Err(Error)
        }

        fn read(&mut self) -> &mut Self {
            self
        }

        fn fix_position(&self, err: Error) -> Error {
            err // Just return the error for the mock
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error
        }
    }

    impl Deserializer for MockDeserializer {
        type Error = Error;

        fn peek_invalid_type(&mut self, exp: &dyn Expected) -> Error {
            let err = match self.peek_or_null().unwrap_or(b'\x00') {
                b'[' => de::Error::invalid_type(Unexpected::Seq, exp),
                _ => self.peek_error(ErrorCode::ExpectedSomeValue),
            };

            self.fix_position(err)
        }
    }

    let mut deserializer = MockDeserializer { current_char: b'[' };
    let expected = MockExpected;

    let result = deserializer.peek_invalid_type(&expected);
    assert_eq!(result, de::Error::invalid_type(Unexpected::Seq, &expected));
}

