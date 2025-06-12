// Answer 0

#[test]
fn test_do_deserialize_u128_valid() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut deserializer = MyDeserializer::new(b"  12345");
    let result = deserializer.do_deserialize_u128(TestVisitor);
    assert_eq!(result, Ok(12345));
}

#[test]
fn test_do_deserialize_u128_negative() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut deserializer = MyDeserializer::new(b"-100");
    let result = deserializer.do_deserialize_u128(TestVisitor);
    assert_eq!(result, Err(deserializer.error(ErrorCode::NumberOutOfRange)));
}

#[test]
fn test_do_deserialize_u128_eof() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut deserializer = MyDeserializer::new(b"  ");
    let result = deserializer.do_deserialize_u128(TestVisitor);
    assert_eq!(result, Err(deserializer.peek_error(ErrorCode::EofWhileParsingValue)));
}

#[test]
fn test_do_deserialize_u128_invalid_number() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut deserializer = MyDeserializer::new(b"12abc");
    let result = deserializer.do_deserialize_u128(TestVisitor);
    assert_eq!(result, Err(deserializer.error(ErrorCode::NumberOutOfRange)));
}

