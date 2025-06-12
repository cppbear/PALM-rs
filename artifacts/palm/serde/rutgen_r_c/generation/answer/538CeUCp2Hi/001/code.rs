// Answer 0

#[test]
fn test_deserialize_seq_invalid_content() {
    struct TestVisitor {
        value: Result<(), String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Self::Error> 
        where
            V: SeqAccess<'de>, 
        {
            Err("Expected sequence".to_string())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Self::Error> {
            self.value
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, Self::Error> {
            self.value
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Self::Error> {
            self.value
        }
        
        // Implement other visit methods as necessary
    }

    let non_seq_content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content: non_seq_content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: Err("Expected sequence".to_string()) };
    
    let result = deserializer.deserialize_seq(visitor);
    
    assert!(result.is_err());
}

#[test]
fn test_deserialize_seq_empty_content() {
    struct TestVisitor {
        value: Result<(), String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Self::Error> 
        where
            V: SeqAccess<'de>, 
        {
            Err("Expected sequence".to_string())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Self::Error> {
            self.value
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, Self::Error> {
            self.value
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Self::Error> {
            self.value
        }

        // Implement other visit methods as necessary
    }

    let non_seq_content = Content::Unit;
    let deserializer = ContentDeserializer {
        content: non_seq_content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: Err("Expected sequence".to_string()) };
    
    let result = deserializer.deserialize_seq(visitor);
    
    assert!(result.is_err());
}

