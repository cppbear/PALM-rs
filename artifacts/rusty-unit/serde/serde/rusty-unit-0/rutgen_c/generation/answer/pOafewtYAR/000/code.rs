// Answer 0

#[test]
fn test_deserialize_string_with_valid_string() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }
        
        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            unreachable!()
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            unreachable!()
        }
    }

    let content = Content::String("test_string".to_owned());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };

    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_string(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_string");
}

#[test]
fn test_deserialize_string_with_wrong_type() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            unreachable!()
        }
        
        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            unreachable!()
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            unreachable!()
        }
    }

    let content = Content::I32(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData::<()>,
    };

    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_string(visitor);

    assert!(result.is_err());
}

