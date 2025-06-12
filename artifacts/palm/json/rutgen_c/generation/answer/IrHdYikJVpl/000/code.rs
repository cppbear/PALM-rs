// Answer 0

#[test]
fn test_deserialize_bytes_valid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let value = Value::Bytes(vec![1, 2, 3, 4]);
    let result: Result<Vec<u8>, Error> = value.deserialize_bytes(TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_deserialize_bytes_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> {
            panic!("Unexpected visit")
        }
    }

    let value = Value::String("invalid".to_string());
    let _result: Result<Vec<u8>, Error> = value.deserialize_bytes(TestVisitor);
}

