// Answer 0

fn test_do_deserialize_u128_ok() {
    struct MockVisitor {
        value: Option<u128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        whitespace: Option<u8>,
        buf: String,
    }

    impl<'de> MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(self.whitespace)
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str(&self.buf);
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer {
        whitespace: Some(b' '),
        buf: "12345678901234567890".to_string(),
    };
    
    let visitor = MockVisitor { value: None };

    let result = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_ok());
}

fn test_do_deserialize_u128_out_of_range() {
    struct MockVisitor {}

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            panic!("Visitor should not be called");
        }
    }

    struct MockDeserializer {
        whitespace: Option<u8>,
        buf: String,
    }

    impl<'de> MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(self.whitespace)
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str(&self.buf);
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer {
        whitespace: Some(b' '),
        buf: "340282366920938463463374607431768211456".to_string(), // Out of u128 range
    };

    let visitor = MockVisitor {};

    let result = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_err());
}

fn test_do_deserialize_u128_negative() {
    struct MockVisitor {}

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            panic!("Visitor should not be called");
        }
    }

    struct MockDeserializer {
        whitespace: Option<u8>,
        buf: String,
    }

    impl<'de> MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(self.whitespace)
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<()> {
            // This function won't be called because of the negative sign
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer {
        whitespace: Some(b'-'), // triggers the negative case
        buf: String::new(),
    };

    let visitor = MockVisitor {};

    let result = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_err());
}

