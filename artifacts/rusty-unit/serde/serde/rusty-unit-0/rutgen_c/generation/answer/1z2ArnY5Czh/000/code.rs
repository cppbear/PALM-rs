// Answer 0

#[test]
fn test_deserialize_bytes_success() {
    struct MockVisitor {
        value: Vec<u8>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value.to_vec())
        }

        // Implement other Visitor methods as no-op or as necessary...
        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(vec![])
        }
    }

    let content = Content::Bytes(b"test".to_vec());
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };
    let visitor = MockVisitor { value: Vec::new() };

    let result = deserializer.deserialize_bytes(visitor);
    assert_eq!(result.unwrap(), b"test".to_vec());
}

#[test]
fn test_deserialize_bytes_invalid_type() {
    struct MockVisitor {
        value: Vec<u8>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, _value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(vec![])
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(vec![])
        }

        // Implement other Visitor methods as no-op or as necessary...
        fn visit_unit(self) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(vec![])
        }
    }

    let content = Content::String("unexpected".into());
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };
    let visitor = MockVisitor { value: Vec::new() };

    let result = deserializer.deserialize_bytes(visitor);
    assert!(result.is_err());
}

