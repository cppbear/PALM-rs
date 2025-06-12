// Answer 0

#[test]
fn test_deserialize_byte_buf_success() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_vec())
        }
    }

    let input: Vec<u8> = vec![1, 2, 3];
    let deserializer = serde_json::Deserializer::from_slice(&input);
    let result: Result<Vec<u8>, _> = deserializer.deserialize_byte_buf(TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_byte_buf_empty() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_vec())
        }
    }

    let input: Vec<u8> = vec![];
    let deserializer = serde_json::Deserializer::from_slice(&input);
    let result: Result<Vec<u8>, _> = deserializer.deserialize_byte_buf(TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

#[should_panic]
#[test]
fn test_deserialize_byte_buf_invalid() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("Expected invalid bytes"))
        }
    }

    let input: Vec<u8> = vec![1, 2, 3];
    let deserializer = serde_json::Deserializer::from_slice(&input);
    let _result: Result<Vec<u8>, _> = deserializer.deserialize_byte_buf(TestVisitor);
}

