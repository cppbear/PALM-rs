// Answer 0

#[test]
fn test_deserialize_byte_buf_string() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string(self, value: String) -> Result<Self::Value, serde::de::Error> {
            Ok(value)
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_owned())
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }
    }

    let content = Content::String("test".to_string());
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<serde::de::value::Error> };

    let result = deserializer.deserialize_byte_buf(TestVisitor { value: None });
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_byte_buf_str() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_owned())
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }
    }

    let content = Content::Str("test");
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<serde::de::value::Error> };

    let result = deserializer.deserialize_byte_buf(TestVisitor { value: None });
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_byte_buf_bytes() {
    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Ok(value)
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_vec())
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }
    }

    let content = Content::Bytes(&[1, 2, 3]);
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<serde::de::value::Error> };

    let result = deserializer.deserialize_byte_buf(TestVisitor { value: None });
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_byte_buf_empty_seq() {
    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }
    }

    let content = Content::Seq(Vec::new());
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<serde::de::value::Error> };

    let result = deserializer.deserialize_byte_buf(TestVisitor { value: None });
    assert!(result.is_err());
}

#[test]
fn test_deserialize_byte_buf_invalid() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_seq<V>(self, _visitor: &mut V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("should not reach here"))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not reach here"))
        }
    }

    let content = Content::Unit;
    let deserializer = ContentDeserializer { content, err: std::marker::PhantomData::<serde::de::value::Error> };

    let result = deserializer.deserialize_byte_buf(TestVisitor { value: None });
    assert!(result.is_err());
}

