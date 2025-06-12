// Answer 0

fn test_peek_invalid_type_map() {
    struct MockExpected;
    
    impl Expected for MockExpected {}

    struct MockDeserializer<'a> {
        input: &'a [u8],
        position: usize,
        scratch: Vec<u8>,
    }

    impl<'a> MockDeserializer<'a> {
        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            // Simulate a successful parse for the test
            Ok(())
        }

        fn parse_any_number(&mut self, _: bool) -> Result<MockNumber, Error> {
            // Return an instance of MockNumber for testing
            Ok(MockNumber)
        }

        fn fix_position(&mut self, err: Error) -> Error {
            // Simulate fixing position and returning the error
            err
        }

        fn peek_error(&mut self, _: ErrorCode) -> Error {
            // Create a simple error to return for testing
            Error
        }

        fn read_str(&mut self, _: &mut Vec<u8>) -> Result<&str, Error> {
            // Simulate a successful string read
            Ok("test")
        }
    }

    struct MockNumber;

    impl MockNumber {
        fn invalid_type(self, _: &dyn Expected) -> Error {
            // Simulate returning a standard Error for invalid type
            Error
        }
    }
    
    let mut deserializer = MockDeserializer {
        input: &[b'{'],
        position: 0,
        scratch: Vec::new(),
    };

    let exp = MockExpected;

    let result = deserializer.peek_invalid_type(&exp);
    // Check result against expected Error output for a valid Map type
}

fn test_peek_invalid_type_number() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct MockDeserializer<'a> {
        input: &'a [u8],
        position: usize,
        scratch: Vec<u8>,
    }

    impl<'a> MockDeserializer<'a> {
        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn parse_any_number(&mut self, _: bool) -> Result<MockNumber, Error> {
            Ok(MockNumber)
        }

        fn fix_position(&mut self, err: Error) -> Error {
            err
        }

        fn peek_error(&mut self, _: ErrorCode) -> Error {
            Error
        }

        fn read_str(&mut self, _: &mut Vec<u8>) -> Result<&str, Error> {
            Ok("test")
        }
    }

    struct MockNumber;

    impl MockNumber {
        fn invalid_type(self, _: &dyn Expected) -> Error {
            Error
        }
    }

    let mut deserializer = MockDeserializer {
        input: &[b'0'], // number character for testing
        position: 0,
        scratch: Vec::new(),
    };

    let exp = MockExpected;

    let result = deserializer.peek_invalid_type(&exp);
    // Check result against expected Error output for a valid Number type
}

