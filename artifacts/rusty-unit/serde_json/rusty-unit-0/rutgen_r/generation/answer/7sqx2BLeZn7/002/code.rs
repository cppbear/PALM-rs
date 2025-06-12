// Answer 0

#[test]
fn test_deserialize_unit_ok() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1; 
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Peek error")
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'_>) -> Error {
            Error::new("Invalid type")
        }
        
        fn fix_position(&self, err: Error) -> Error {
            err
        }
        
        fn parse_ident(&mut self, _word: &[u8]) -> Result<()> {
            Ok(())
        }
        
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }
    }

    let mut deserializer = TestDeserializer::new(b" null".to_vec());
    let result = deserializer.deserialize_unit(Visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_unit_invalid_type() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Peek error")
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'_>) -> Error {
            Error::new("Invalid type")
        }
        
        fn fix_position(&self, err: Error) -> Error {
            err
        }
        
        fn parse_ident(&mut self, _word: &[u8]) -> Result<()> {
            // Mock an invalid type and do not consume
            Err(Error::new("Not a valid identifier"))
        }
        
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }
    }

    let mut deserializer = TestDeserializer::new(b" x".to_vec());
    let result = deserializer.deserialize_unit(Visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_unit_eof() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1; 
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Peek error")
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'_>) -> Error {
            Error::new("Invalid type")
        }
        
        fn fix_position(&self, err: Error) -> Error {
            err
        }
        
        fn parse_ident(&mut self, _word: &[u8]) -> Result<()> {
            Ok(())
        }
        
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }
    }

    let mut deserializer = TestDeserializer::new(b"".to_vec());
    let result = deserializer.deserialize_unit(Visitor);
    assert!(result.is_err());
}

