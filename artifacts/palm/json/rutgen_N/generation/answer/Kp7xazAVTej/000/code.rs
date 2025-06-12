// Answer 0

#[test]
fn test_deserialize_byte_buf_valid() {
    struct DummyVisitor {
        value: Vec<u8>,
    }

    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
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

    let input_data: &[u8] = &[1, 2, 3];
    let result: Result<Vec<u8>, serde_json::Error> = serde_json::Deserializer::from_slice(input_data).deserialize_byte_buf(DummyVisitor { value: Vec::new() });
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_byte_buf_invalid() {
    struct DummyVisitor;

    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("invalid byte buffer"))
        }
    }

    let input_data: &[u8] = &[1, 2, 3];
    let _: Result<Vec<u8>, serde_json::Error> = serde_json::Deserializer::from_slice(input_data).deserialize_byte_buf(DummyVisitor);
}

