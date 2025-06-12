// Answer 0

#[test]
fn test_do_deserialize_i128_positive() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut deserializer = MyDeserializer::new(b"123".to_vec());
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert_eq!(result, Ok(123));
}

#[test]
fn test_do_deserialize_i128_negative() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut deserializer = MyDeserializer::new(b"-456".to_vec());
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert_eq!(result, Ok(-456));
}

#[test]
fn test_do_deserialize_i128_eof() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            unreachable!()
        }
    }

    let mut deserializer = MyDeserializer::new(b"".to_vec()); // empty input
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_do_deserialize_i128_out_of_range() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            unreachable!()
        }
    }

    let large_value = b"170141183460469231731687303715884105727"; // exceeds i128 max
    let mut deserializer = MyDeserializer::new(large_value.to_vec());
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
}

