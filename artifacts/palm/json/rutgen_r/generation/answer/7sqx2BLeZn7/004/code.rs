// Answer 0

#[test]
fn test_deserialize_unit_returns_err_on_eof() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        error_on_parse: bool,
        error_on_ident: bool,
    }
    
    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.error_on_parse {
                Err(ErrorCode::EofWhileParsingValue)
            } else {
                Ok(Some(b'n'))
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            if self.error_on_ident {
                Err(ErrorCode::InvalidValue)
            } else {
                Ok(())
            }
        }

        fn peek_error(&self, error: ErrorCode) -> Result<()> {
            Err(error)
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'de>) -> Result<()> {
            Err(ErrorCode::InvalidType)
        }

        fn fix_position<T>(&self, result: Result<T>) -> Result<T> {
            result
        }
    }

    let mut deserializer = TestDeserializer {
        error_on_parse: true,
        error_on_ident: false,
    };
    
    let result = deserializer.deserialize_unit(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_unit_returns_err_on_invalid_ident() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        error_on_parse: bool,
        error_on_ident: bool,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.error_on_parse {
                Err(ErrorCode::EofWhileParsingValue)
            } else {
                Ok(Some(b'n'))
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            if self.error_on_ident {
                Err(ErrorCode::InvalidValue)
            } else {
                Ok(())
            }
        }

        fn peek_error(&self, error: ErrorCode) -> Result<()> {
            Err(error)
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'de>) -> Result<()> {
            Err(ErrorCode::InvalidType)
        }

        fn fix_position<T>(&self, result: Result<T>) -> Result<T> {
            result
        }
    }

    let mut deserializer = TestDeserializer {
        error_on_parse: false,
        error_on_ident: true,
    };

    let result = deserializer.deserialize_unit(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_unit_success() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        error_on_parse: bool,
        error_on_ident: bool,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.error_on_parse {
                Err(ErrorCode::EofWhileParsingValue)
            } else {
                Ok(Some(b'n'))
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            if self.error_on_ident {
                Err(ErrorCode::InvalidValue)
            } else {
                Ok(())
            }
        }

        fn peek_error(&self, error: ErrorCode) -> Result<()> {
            Err(error)
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'de>) -> Result<()> {
            Err(ErrorCode::InvalidType)
        }

        fn fix_position<T>(&self, result: Result<T>) -> Result<T> {
            result
        }
    }

    let mut deserializer = TestDeserializer {
        error_on_parse: false,
        error_on_ident: false,
    };

    let result = deserializer.deserialize_unit(TestVisitor);
    assert!(result.is_ok());
}

