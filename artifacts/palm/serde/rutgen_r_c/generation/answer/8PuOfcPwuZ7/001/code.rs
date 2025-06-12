// Answer 0

#[test]
fn test_deserialize_byte_buf_with_valid_bytes() {
    struct TestVisitor {
        received: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got str"))
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got str"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got unit"))
        }
    }

    let content = Content::Bytes(vec![1, 2, 3, 4]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor { received: None };
    let result = deserializer.deserialize_byte_buf(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn test_deserialize_byte_buf_with_string() {
    struct TestVisitor {
        received: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got str"))
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got str"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got unit"))
        }
    }

    let content = Content::String("test".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor { received: None };
    let result = deserializer.deserialize_byte_buf(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_byte_buf_with_invalid_type() {
    struct TestVisitor {
        received: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got str"))
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got str"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Expected bytes, got unit"))
        }
    }

    let content = Content::Unit;
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    let visitor = TestVisitor { received: None };
    let result = deserializer.deserialize_byte_buf(visitor);
    assert!(result.is_err());
}

