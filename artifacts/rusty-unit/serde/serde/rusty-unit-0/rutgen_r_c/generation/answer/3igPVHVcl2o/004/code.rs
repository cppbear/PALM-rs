// Answer 0

#[test]
fn test_deserialize_str_with_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_str<V>(self, value: &str) -> Result<Self::Value, E> {
            assert_eq!(value, "test_string");
            Ok(value)
        }

        fn visit_borrowed_str<V>(self, value: &'de str) -> Result<Self::Value, E> {
            assert_eq!(value, "test_borrowed_string");
            Ok(value)
        }

        fn visit_bytes<V>(self, value: &[u8]) -> Result<Self::Value, E> {
            panic!("visit_bytes should not be called");
        }

        fn visit_borrowed_bytes<V>(self, value: &[u8]) -> Result<Self::Value, E> {
            panic!("visit_borrowed_bytes should not be called");
        }
    }

    let content = Content::String("test_string".to_string());
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let result = deserializer.deserialize_str(TestVisitor);
    assert!(result.is_ok());

    let content_borrowed = Content::Str("test_borrowed_string");
    let deserializer_borrowed = ContentRefDeserializer { content: &content_borrowed, err: PhantomData };
    let result_borrowed = deserializer_borrowed.deserialize_str(TestVisitor);
    assert!(result_borrowed.is_ok());
}

#[test]
fn test_deserialize_str_with_bytes() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'de [u8];

        fn visit_str<V>(self, value: &str) -> Result<Self::Value, E> {
            panic!("visit_str should not be called");
        }

        fn visit_borrowed_str<V>(self, value: &'de str) -> Result<Self::Value, E> {
            panic!("visit_borrowed_str should not be called");
        }

        fn visit_bytes<V>(self, value: &[u8]) -> Result<Self::Value, E> {
            assert_eq!(value, b"test_bytes");
            Ok(value)
        }

        fn visit_borrowed_bytes<V>(self, value: &[u8]) -> Result<Self::Value, E> {
            assert_eq!(value, b"test_borrowed_bytes");
            Ok(value)
        }
    }

    let content = Content::ByteBuf(b"test_bytes".to_vec());
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let result = deserializer.deserialize_str(TestVisitor);
    assert!(result.is_err());

    let content_borrowed = Content::Bytes(b"test_borrowed_bytes");
    let deserializer_borrowed = ContentRefDeserializer { content: &content_borrowed, err: PhantomData };
    let result_borrowed = deserializer_borrowed.deserialize_str(TestVisitor);
    assert!(result_borrowed.is_err());
}

