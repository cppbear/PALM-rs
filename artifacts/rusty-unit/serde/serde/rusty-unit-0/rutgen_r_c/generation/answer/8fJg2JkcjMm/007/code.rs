// Answer 0

#[test]
fn test_deserialize_any_with_bytes() {
    use crate::de::Visitor;

    struct TestVisitor {
        visited_bytes: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(vec![])
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(vec![])
        }

        // Other Visitor methods can be ignored for this test
    }

    let content_bytes = Content::Bytes(vec![1, 2, 3, 4]);
    let deserializer = ContentRefDeserializer::new(&content_bytes);
    let visitor = TestVisitor { visited_bytes: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn test_deserialize_any_with_borrowed_bytes() {
    use crate::de::Visitor;

    struct TestVisitor {
        visited_bytes: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(vec![])
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(vec![])
        }

        // Other Visitor methods can be ignored for this test
    }

    let content_borrowed_bytes = Content::Bytes(&[5, 6, 7, 8]);
    let deserializer = ContentRefDeserializer::new(&content_borrowed_bytes);
    let visitor = TestVisitor { visited_bytes: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![5, 6, 7, 8]);
}

