// Answer 0

#[test]
fn test_do_deserialize_u128_negative_number() {
    struct MockVisitor;
    
    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = Result<u128, ErrorCode>;
        
        fn visit_u128(self, value: u128) -> Self::Value {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: String,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate parsing whitespace and seeing a negative sign
            if self.input.starts_with('-') {
                Ok(Some(b'-'))
            } else {
                Ok(Some(b' ')) // not negative
            }
        }
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ErrorCode> {
            // Simulate scanning an integer
            if self.input.contains('-') {
                return Err(ErrorCode::NumberOutOfRange);
            }
            buf.push_str("12345678901234567890");
            Ok(())
        }
        
        fn peek_error(&self, code: ErrorCode) -> Error {
            Error { code }
        }
        
        fn error(&self, code: ErrorCode) -> Error {
            Error { code }
        }
        
        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer { input: String::from("-123") };
    let visitor = MockVisitor;
    let result = deserializer.do_deserialize_u128(visitor);

    assert_eq!(result, Err(deserializer.peek_error(ErrorCode::NumberOutOfRange)));
}

#[test]
fn test_do_deserialize_u128_eof_error() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = Result<u128, ErrorCode>;

        fn visit_u128(self, value: u128) -> Self::Value {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: String,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate EOF scenario
            Ok(None)
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ErrorCode> {
            buf.push_str("12345678901234567890");
            Ok(())
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error { code }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error { code }
        }
        
        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer { input: String::from("") };
    let visitor = MockVisitor;
    let result = deserializer.do_deserialize_u128(visitor);

    assert_eq!(result, Err(deserializer.peek_error(ErrorCode::EofWhileParsingValue)));
}

