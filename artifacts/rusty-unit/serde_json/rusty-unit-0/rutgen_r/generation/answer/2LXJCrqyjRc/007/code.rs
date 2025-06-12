// Answer 0

#[test]
fn test_do_deserialize_i128_valid_negative() {
    struct MockVisitor;
    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;
        
        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }
    
    struct MockDeserializer {
        buf: String,
        error: Option<ErrorCode>,
    }
    
    impl MockDeserializer {
        fn new() -> Self {
            Self { buf: String::new(), error: None }
        }
        
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'-'))
        }
        
        fn eat_char(&mut self) {}
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str("12345678901234567890");
            Ok(())
        }
        
        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::NumberOutOfRange
        }
        
        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err
        }
    }
    
    let mut deserializer = MockDeserializer::new();
    let visitor = MockVisitor;
    
    let result = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12345678901234567890);
}

#[test]
fn test_do_deserialize_i128_invalid_number() {
    struct MockVisitor;
    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;
        
        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    struct MockDeserializer {
        buf: String,
    }
    
    impl MockDeserializer {
        fn new() -> Self {
            Self { buf: String::new() }
        }
        
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'-'))
        }
        
        fn eat_char(&mut self) {}
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str("12345678901234567890");
            Ok(())
        }
        
        fn error(&self, code: ErrorCode) -> ErrorCode {
            code
        }
        
        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err
        }
    }
    
    let mut deserializer = MockDeserializer::new();
    let visitor = MockVisitor;
    
    let result = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::NumberOutOfRange);
}

#[test]
fn test_do_deserialize_i128_eof() {
    struct MockVisitor;
    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;
        
        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    struct MockDeserializer {
        buf: String,
    }
    
    impl MockDeserializer {
        fn new() -> Self {
            Self { buf: String::new() }
        }
        
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(None) // Simulating EOF
        }
        
        fn eat_char(&mut self) {}
        
        fn scan_integer128(&mut self, _buf: &mut String) -> Result<()> {
            Ok(())
        }
        
        fn error(&self, code: ErrorCode) -> ErrorCode {
            code
        }
        
        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err
        }
    }
    
    let mut deserializer = MockDeserializer::new();
    let visitor = MockVisitor;
    
    let result = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::EofWhileParsingValue);
}

