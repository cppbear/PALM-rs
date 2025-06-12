// Answer 0

#[test]
fn test_deserialize_bool_valid() {
    struct TestVisitor {
        value: Option<bool>,
    }
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;
        
        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("Expected a bool, not a unit");
        }
        
        // Implement other required visitor methods...
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };
    
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_bool(visitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_invalid() {
    struct TestVisitor {
        value: Option<bool>,
    }
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = bool;
        
        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("Expected a bool, not a unit");
        }
        
        // Implement other required visitor methods...
    }

    let content = Content::I32(42);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };
    
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_bool(visitor);
    assert!(result.is_err());
}

