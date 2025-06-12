// Answer 0

#[test]
fn test_deserialize_i64_valid() {
    struct MockVisitor {
        value: Option<i64>,
    }
    
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = i64;
        
        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Err(E::custom("Expected i64, got bool"))
        }
        
        // Other visit methods can return unimplemented to keep it minimal
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got u64")) }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got f32")) }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got char")) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got str")) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got bytes")) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got unit")) }
    }

    let content = Content::I64(123);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };

    let visitor = MockVisitor { value: None };
    let result = deserializer.deserialize_i64(visitor);
    assert_eq!(result, Ok(123));
}

#[test]
fn test_deserialize_i64_invalid_type() {
    struct MockVisitor {
        value: Option<i64>,
    }
    
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = i64;
        
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Should not have been called"))
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            Err(E::custom("Expected i64, got bool"))
        }
        
        // Other visit methods can return unimplemented to keep it minimal
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got u64")) }
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got f32")) }
        fn visit_char<E>(self, _: char) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got char")) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got str")) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got bytes")) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(E::custom("Expected i64, got unit")) }
    }

    let content = Content::Bool(false);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };

    let visitor = MockVisitor { value: None };
    let result = deserializer.deserialize_i64(visitor);
    assert!(result.is_err());
}

