// Answer 0

#[test]
fn test_deserialize_map_valid() {
    struct MockVisitor {
        visited: bool,
    }
    
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error> {
            // Simulate visiting a map successfully
            Ok(())
        }
        
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            self.visited = true;
            Ok(())
        }

        // Other methods would be unimplemented in this example
    }

    let content = Content::Map(vec![
        (Content::String("key".to_string()), Content::U8(10)),
    ]);
    
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<()>,
    };

    let visitor = MockVisitor { visited: false };
    
    let result = deserializer.deserialize_map(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_map_invalid_type() {
    struct MockVisitor {
        visited: bool,
    }
    
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error> {
            // Visitor should not reach here
            unreachable!();
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            self.visited = true;
            Ok(())
        }

        // Other methods would be unimplemented in this example
    }

    let content = Content::U8(10); // Invalid type
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<()>,
    };

    let visitor = MockVisitor { visited: false };

    let result = deserializer.deserialize_map(visitor);
    assert!(result.is_err());
}

