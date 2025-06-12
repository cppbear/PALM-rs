// Answer 0

#[test]
fn test_deserialize_any_f32() {
    use crate::Visitor;
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            assert_eq!(value, 3.14);
            Ok(value)
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("should not reach visit_unit");
        }
        
        fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
            panic!("should not reach visit_bool");
        }
        
        fn visit_i8<E>(self, _value: i8) -> Result<Self::Value, E> {
            panic!("should not reach visit_i8");
        }

        fn visit_u8<E>(self, _value: u8) -> Result<Self::Value, E> {
            panic!("should not reach visit_u8");
        }

        fn visit_char<E>(self, _value: char) -> Result<Self::Value, E> {
            panic!("should not reach visit_char");
        }

        // Implement other required traits with panics or no-ops
        fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
            panic!("should not reach visit_string");
        }

        fn visit_newtype_struct<E>(self, _deserializer: ContentDeserializer<'de, E>) -> Result<Self::Value, E> {
            panic!("should not reach visit_newtype_struct");
        }

        fn visit_seq<E>(self, _deserializer: &mut dyn SeqAccess<'de>) -> Result<Self::Value, E> {
            panic!("should not reach visit_seq");
        }

        fn visit_map<E>(self, _deserializer: &mut dyn MapAccess<'de>) -> Result<Self::Value, E> {
            panic!("should not reach visit_map");
        }

        // Add other required visitor methods
        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> {
            panic!("should not reach visit_borrowed_str");
        }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentDeserializer::new(content);
    
    let result: f32 = deserializer.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, 3.14);
}

#[test]
fn test_deserialize_any_invalid_type() {
    use crate::Visitor;
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn visit_i32<E>(self, _value: i32) -> Result<Self::Value, E> {
            panic!("should not reach visit_i32");
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("should not reach visit_unit");
        }
        
        fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
            panic!("should not reach visit_bool");
        }

        fn visit_f32<E>(self, _value: f32) -> Result<Self::Value, E> {
            panic!("should not reach visit_f32");
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
            panic!("should not reach visit_str");
        }

        // Implement other required traits with panics or no-ops
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer::new(content);
    
    let result = deserializer.deserialize_any(TestVisitor);
    assert!(result.is_err());
}

