// Answer 0

#[test]
fn test_deserialize_byte_buf_byte_buf() {
    struct MockVisitor {
        result: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<Vec<u8>>;

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value))
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value.to_vec()))
        }

        fn visit_none(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }
        
        fn visit_unit(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }

        fn visit_string(self, _: String) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }
    }

    let content = Content::ByteBuf(vec![1, 2, 3, 4]);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };

    let result = deserializer.deserialize_byte_buf(MockVisitor { result: None }).unwrap();

    assert_eq!(result, Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_deserialize_byte_buf_string() {
    struct MockVisitor {
        result: Option<String>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<String>;

        fn visit_string(self, value: String) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value))
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }

        fn visit_none(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }

        fn visit_unit(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }
        
        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }
    }

    let content = Content::String("test".to_string());
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };

    let result = deserializer.deserialize_byte_buf(MockVisitor { result: None }).unwrap();

    assert_eq!(result, Some("test".to_string()));
}

#[test]
fn test_deserialize_byte_buf_bytes() {
    struct MockVisitor {
        result: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<Vec<u8>>;

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, std::convert::Infallible> {
            Ok(Some(value.to_vec()))
        }

        fn visit_none(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }

        fn visit_unit(self) -> Result<Self::Value, std::convert::Infallible> {
            Ok(None)
        }
        
        fn visit_string(self, _: String) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }

        fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, std::convert::Infallible> {
            unimplemented!()
        }
    }

    let content = Content::Bytes(&[5, 6, 7, 8][..]);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData };

    let result = deserializer.deserialize_byte_buf(MockVisitor { result: None }).unwrap();

    assert_eq!(result, Some(vec![5, 6, 7, 8]));
}

