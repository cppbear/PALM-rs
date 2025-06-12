// Answer 0

#[test]
fn test_deserialize_any_i64() {
    struct VisitorImpl {
        value: Option<i64>,
    }

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = i64;

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Err(E::custom("Expected i64"))
        }

        // Implement other visitor methods if needed (omitting for brevity)
    }

    let content = Content::I64(42);
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = VisitorImpl { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_any_i64_invalid_type() {
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = i64;

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unreachable!()
        }

        fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Err(E::custom("Expected i64, got bool"))
        }
        
        // Implement other visitor methods if needed
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer::new(&content);
    let visitor = VisitorImpl;

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

