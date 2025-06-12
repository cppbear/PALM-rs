// Answer 0

#[test]
fn test_deserialize_bytes_with_bytes_content() {
    use crate::de::Visitor;
    use crate::de::Error;

    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, value: &'de [u8]) -> Result<Self::Value, Error> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Error> {
            Ok(value.to_vec())
        }

        fn visit_str(self, _: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("expected bytes"))
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("expected bytes"))
        }
    }

    let content = Content::Bytes(vec![1, 2, 3, 4]);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_bytes(TestVisitor { value: None });
    
    assert_eq!(result, Ok(vec![1, 2, 3, 4]));
}

#[test]
fn test_deserialize_bytes_with_byte_buf_content() {
    use crate::de::Visitor;
    use crate::de::Error;

    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, value: &'de [u8]) -> Result<Self::Value, Error> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Error> {
            Ok(value.to_vec())
        }

        fn visit_str(self, _: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("expected bytes"))
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("expected bytes"))
        }
    }

    let content = Content::ByteBuf(vec![5, 6, 7]);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_bytes(TestVisitor { value: None });
    
    assert_eq!(result, Ok(vec![5, 6, 7]));
}

