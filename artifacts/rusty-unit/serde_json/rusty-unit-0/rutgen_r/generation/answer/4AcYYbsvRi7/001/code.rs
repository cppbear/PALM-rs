// Answer 0

#[test]
fn test_deserialize_byte_buf_success() {
    struct TestVisitor {
        value: Option<Vec<u8>>,
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

    let byte_buf: &[u8] = &[1, 2, 3, 4, 5];
    let result = deserialize_byte_buf(byte_buf, TestVisitor { value: None }).unwrap();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
#[should_panic]
fn test_deserialize_byte_buf_empty() {
    struct TestVisitor {
        value: Option<Vec<u8>>,
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
            if value.is_empty() {
                panic!("Empty buffer cannot be deserialized!");
            }
            Ok(value.to_vec())
        }
    }

    let byte_buf: &[u8] = &[];
    deserialize_byte_buf(byte_buf, TestVisitor { value: None }).unwrap();
}

#[test]
fn test_deserialize_byte_buf_non_ascii() {
    struct TestVisitor {
        value: Option<Vec<u8>>,
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

    let byte_buf: &[u8] = &[0, 255, 128, 64];
    let result = deserialize_byte_buf(byte_buf, TestVisitor { value: None }).unwrap();
    assert_eq!(result, vec![0, 255, 128, 64]);
}

