// Answer 0

#[test]
fn test_do_deserialize_u128_negative_value() {
    struct TestVisitor;

    impl de::Visitor<'_> for TestVisitor {
        type Value = Result<u128, Error>;

        fn visit_u128<E>(self, value: u128) -> Result<u128, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        offset: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            TestDeserializer { input, offset: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, Error> {
            if self.offset < self.input.len() {
                let byte = self.input[self.offset];
                self.offset += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), Error> {
            buf.push_str("-123"); // simulate negative number
            Ok(())
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code, "Number out of range")
        }
    }

    let mut deserializer = TestDeserializer::new(b" \n\t-123".to_vec());
    let result = deserializer.do_deserialize_u128(TestVisitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::NumberOutOfRange);
}

#[test]
fn test_do_deserialize_u128_eof_error() {
    struct TestVisitor;

    impl de::Visitor<'_> for TestVisitor {
        type Value = Result<u128, Error>;

        fn visit_u128<E>(self, value: u128) -> Result<u128, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        offset: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            TestDeserializer { input, offset: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, Error> {
            if self.offset < self.input.len() {
                let byte = self.input[self.offset];
                self.offset += 1;
                Ok(Some(byte))
            } else {
                Ok(None) // Simulating EOF
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), Error> {
            Ok(()) // No value read, simulating EOF
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code, "EOF while parsing value")
        }
    }

    let mut deserializer = TestDeserializer::new(b" \n\t".to_vec());
    let result = deserializer.do_deserialize_u128(TestVisitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::EofWhileParsingValue);
}

