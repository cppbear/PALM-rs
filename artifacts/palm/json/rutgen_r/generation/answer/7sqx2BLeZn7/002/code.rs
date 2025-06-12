// Answer 0

#[test]
fn test_deserialize_unit_valid() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        peek_char: Option<u8>,
        parse_whitespace_result: Result<usize>,
    }

    impl MockDeserializer {
        fn new(peek_char: Option<u8>, parse_whitespace_result: Result<usize>) -> Self {
            Self {
                peek_char,
                parse_whitespace_result,
            }
        }

        fn parse_whitespace(&mut self) -> Result<usize> {
            self.parse_whitespace_result.clone()
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type(&self, _visitor: &MockVisitor) -> Result<()> {
            Err(ErrorCode::TypeMismatch.into())
        }

        fn eat_char(&mut self) {
            self.peek_char = None;
        }

        fn parse_ident(&self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: Result<()>) -> Result<()> {
            err
        }
    }

    let mut deserializer = MockDeserializer::new(Some(b'n'), Ok(1));
    let visitor = MockVisitor;
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_unit_invalid_peek() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        peek_char: Option<u8>,
        parse_whitespace_result: Result<usize>,
    }

    impl MockDeserializer {
        fn new(peek_char: Option<u8>, parse_whitespace_result: Result<usize>) -> Self {
            Self {
                peek_char,
                parse_whitespace_result,
            }
        }

        fn parse_whitespace(&mut self) -> Result<usize> {
            self.parse_whitespace_result.clone()
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type(&self, _visitor: &MockVisitor) -> Result<()> {
            Err(ErrorCode::TypeMismatch.into())
        }

        fn eat_char(&mut self) {
            self.peek_char = None;
        }

        fn parse_ident(&self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: Result<()>) -> Result<()> {
            err
        }
    }

    let mut deserializer = MockDeserializer::new(Some(b'x'), Ok(1));
    let visitor = MockVisitor;
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_unit_whitespace_error() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        parse_whitespace_result: Result<usize>,
    }

    impl MockDeserializer {
        fn new(parse_whitespace_result: Result<usize>) -> Self {
            Self {
                parse_whitespace_result,
            }
        }

        fn parse_whitespace(&mut self) -> Result<usize> {
            self.parse_whitespace_result.clone()
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type(&self, _visitor: &MockVisitor) -> Result<()> {
            Err(ErrorCode::TypeMismatch.into())
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: Result<()>) -> Result<()> {
            err
        }
    }

    let mut deserializer = MockDeserializer::new(Err(ErrorCode::ParsingError));
    let visitor = MockVisitor;
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_err());
}

