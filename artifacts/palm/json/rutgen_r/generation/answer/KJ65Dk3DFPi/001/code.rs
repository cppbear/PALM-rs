// Answer 0

#[test]
fn test_deserialize_bytes_with_valid_visitor() {
    struct TestVisitor {
        value: Vec<u8>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte array")
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_vec())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Expected bytes but got string"))
        }
    }

    let de = serde_json::Deserializer::from_slice(b"[1, 2, 3]");
    let visitor = TestVisitor { value: Vec::new() };
    let result = de.deserialize_bytes(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "Expected bytes but got string")]
fn test_deserialize_bytes_with_invalid_visitor() {
    struct InvalidVisitor;

    impl<'de> de::Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte array or valid representation")
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
        
        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Expected bytes but got string"))
        }
    }

    let de = serde_json::Deserializer::from_slice(b"\"not byte data\"");
    let visitor = InvalidVisitor;
    let _ = de.deserialize_bytes(visitor);
}

