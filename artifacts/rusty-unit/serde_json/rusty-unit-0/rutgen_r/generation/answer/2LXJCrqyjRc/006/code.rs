// Answer 0

fn test_do_deserialize_i128_success() {
    struct Visitor;
    impl<'a> serde_json::de::Visitor<'a> for Visitor {
        type Value = i128;
        
        fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: &'static str,
    }

    impl TestDeserializer {
        fn new(input: &'static str) -> Self {
            Self { input }
        }

        // Simulates parsing whitespace, returns `Ok(val)` or `Err(err)`
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.input.chars().next().unwrap_or_default().is_whitespace() {
                self.input = &self.input[1..]; // Consume whitespace
                Ok(Some(b'0')) // Simulate a character following whitespace
            } else {
                Err(ErrorCode::EofWhileParsingValue) // Simulate EOF
            }
        }

        // Simulates scanning an i128 number
        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ErrorCode> {
            let number = self.input.chars().take_while(|c| c.is_digit(10)).collect::<String>();
            if number.is_empty() {
                return Err(ErrorCode::EofWhileParsingValue); 
            }
            buf.push_str(&number);
            self.input = &self.input[number.len()..]; // Consume scanned characters
            Ok(())
        }

        // Placeholder for error handling
        fn peek_error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::EofWhileParsingValue
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::NumberOutOfRange
        }

        fn fix_position<E>(&self, err: E) -> E {
            err
        }
    }

    let mut deserializer = TestDeserializer::new(" 123");
    let result = deserializer.do_deserialize_i128(Visitor);
    assert_eq!(result.ok(), Some(123));
}

fn test_do_deserialize_i128_negative() {
    struct Visitor;
    impl<'a> serde_json::de::Visitor<'a> for Visitor {
        type Value = i128;
        
        fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: &'static str,
    }

    impl TestDeserializer {
        fn new(input: &'static str) -> Self {
            Self { input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.input.chars().next().unwrap_or_default().is_whitespace() {
                self.input = &self.input[1..];
                Ok(Some(b'-')) // Simulate a negative sign
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ErrorCode> {
            let number = self.input.chars().take_while(|c| c.is_digit(10)).collect::<String>();
            if number.is_empty() {
                return Err(ErrorCode::EofWhileParsingValue);
            }
            buf.push_str(&number);
            self.input = &self.input[number.len()..];
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::EofWhileParsingValue
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::NumberOutOfRange
        }

        fn fix_position<E>(&self, err: E) -> E {
            err
        }
    }

    let mut deserializer = TestDeserializer::new(" 123");
    let result = deserializer.do_deserialize_i128(Visitor);
    assert_eq!(result.ok(), Some(-123));
}

fn test_do_deserialize_i128_eof_error() {
    struct Visitor;
    impl<'a> serde_json::de::Visitor<'a> for Visitor {
        type Value = i128;
        
        fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E> {
            Err(E::from_error_code(ErrorCode::EofWhileParsingValue))
        }
    }

    struct TestDeserializer {
        input: &'static str,
    }

    impl TestDeserializer {
        fn new(input: &'static str) -> Self {
            Self { input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.input.is_empty() {
                return Ok(None);
            }
            self.input = &self.input[1..];
            Ok(Some(b' ')) // Return whitespace
        }

        fn scan_integer128(&mut self, _buf: &mut String) -> Result<(), ErrorCode> {
            Err(ErrorCode::EofWhileParsingValue)
        }

        fn peek_error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::EofWhileParsingValue
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::NumberOutOfRange
        }

        fn fix_position<E>(&self, err: E) -> E {
            err
        }
    }

    let mut deserializer = TestDeserializer::new(" ");
    let result = deserializer.do_deserialize_i128(Visitor);
    assert!(result.is_err());
}

fn test_do_deserialize_i128_number_out_of_range() {
    struct Visitor;
    impl<'a> serde_json::de::Visitor<'a> for Visitor {
        type Value = i128;
        
        fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E> {
            Err(E::from_error_code(ErrorCode::NumberOutOfRange))
        }
    }

    struct TestDeserializer {
        input: &'static str,
    }

    impl TestDeserializer {
        fn new(input: &'static str) -> Self {
            Self { input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.input.chars().next().unwrap_or_default().is_whitespace() {
                self.input = &self.input[1..];
                Ok(Some(b'0')) // Simulate whitespace parsed
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn scan_integer128(&mut self, _buf: &mut String) -> Result<(), ErrorCode> {
            Err(ErrorCode::NumberOutOfRange)
        }

        fn peek_error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::EofWhileParsingValue
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::NumberOutOfRange
        }

        fn fix_position<E>(&self, err: E) -> E {
            err
        }
    }

    let mut deserializer = TestDeserializer::new(" 0");
    let result = deserializer.do_deserialize_i128(Visitor);
    assert!(result.is_err());
}

