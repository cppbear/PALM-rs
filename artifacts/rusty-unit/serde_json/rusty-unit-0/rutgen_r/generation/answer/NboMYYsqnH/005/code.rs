// Answer 0

#[test]
fn test_do_deserialize_u128_success() {
    struct TestVisitor {
        value: Option<u128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        buf: String,
        whitespace_return: Option<u8>,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(self.whitespace_return)
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ErrorCode> {
            buf.push_str(&self.buf);
            Ok(())
        }
        
        fn new(buf: &str, whitespace: Option<u8>) -> Self {
            Self {
                buf: buf.to_string(),
                whitespace_return: whitespace,
            }
        }
    }

    let mut deserializer = TestDeserializer::new("1234567890123456789012345678901234567890", Some(b' '));
    let visitor = TestVisitor { value: None };
    let result: Result<u128, _> = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Ok(1234567890123456789012345678901234567890));
}

#[test]
#[should_panic]
fn test_do_deserialize_u128_invalid_negative() {
    struct TestDeserializer {
        whitespace_return: Option<u8>,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(Some(b'-'))
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn new(whitespace: Option<u8>) -> Self {
            Self { whitespace_return: whitespace }
        }
    }

    let mut deserializer = TestDeserializer::new(Some(b'-'));
    let visitor = TestVisitor { value: None };
    let _result: Result<u128, _> = deserializer.do_deserialize_u128(visitor);
}

#[test]
fn test_do_deserialize_u128_eof() {
    struct TestVisitor {
        value: Option<u128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            Ok(0)
        }
    }

    struct TestDeserializer {
        whitespace_return: Option<u8>,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(None)
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn new(whitespace: Option<u8>) -> Self {
            Self { whitespace_return: whitespace }
        }
    }

    let mut deserializer = TestDeserializer::new(None);
    let visitor = TestVisitor { value: None };
    let result: Result<u128, _> = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_err());
}

#[test]
fn test_do_deserialize_u128_parse_error() {
    struct TestVisitor {
        value: Option<u128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E> {
            Ok(0)
        }
    }

    struct TestDeserializer {
        buf: String,
        whitespace_return: Option<u8>,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(Some(b' '))
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ErrorCode> {
            buf.push_str("not_a_number");
            Ok(())
        }
        
        fn new(buf: &str, whitespace: Option<u8>) -> Self {
            Self {
                buf: buf.to_string(),
                whitespace_return: whitespace,
            }
        }
    }

    let mut deserializer = TestDeserializer::new("not_a_number", Some(b' '));
    let visitor = TestVisitor { value: None };
    let result: Result<u128, _> = deserializer.do_deserialize_u128(visitor);
    assert!(result.is_err());
}

