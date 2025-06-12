// Answer 0

#[test]
fn test_deserialize_byte_buf_success() {
    struct TestVisitor {
        value: Vec<u8>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_vec())
        }
    }

    let test_data: &[u8] = &[1, 2, 3];
    let deserializer = serde_json::Deserializer::from_slice(test_data);
    let visitor = TestVisitor { value: vec![] };
    let result: Result<Vec<u8>> = deserializer.deserialize_bytes(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_byte_buf_invalid() {
    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            panic!("Invalid bytes");
        }
    }

    let test_data: &[u8] = b"invalid";
    let deserializer = serde_json::Deserializer::from_slice(test_data);
    let visitor = InvalidVisitor;
    let _result: Result<Vec<u8>> = deserializer.deserialize_bytes(visitor);
}

