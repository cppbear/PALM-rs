// Answer 0

#[test]
fn test_deserialize_string_with_string_content() {
    use crate::de::Visitor;
    use crate::de::value::Content;
    use crate::de::Error;
    
    struct MockVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<String>;

        fn visit_string(self, value: String) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("borrowed str not supported"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, Error> {
            Err(Error::custom("byte buf not supported"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, Error> {
            Err(Error::custom("borrowed bytes not supported"))
        }

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Err(Error::custom("unit not supported"))
        }

        fn visit_none(self) -> Result<Self::Value, Error> {
            Err(Error::custom("none not supported"))
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Error> {
            Err(Error::custom("some not supported"))
        }
    }

    let content = Content::String("test_string".to_string());
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<Error>,
    };
    let result = deserializer.deserialize_string(MockVisitor { value: None }).unwrap();
    assert_eq!(result, Some("test_string".to_string()));
}

#[test]
fn test_deserialize_string_with_str_content() {
    use crate::de::Visitor;
    use crate::de::value::Content;
    use crate::de::Error;
    
    struct MockVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<String>;

        fn visit_string(self, _value: String) -> Result<Self::Value, Error> {
            Err(Error::custom("string not supported"))
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, Error> {
            Ok(Some(value.to_string()))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, Error> {
            Err(Error::custom("byte buf not supported"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, Error> {
            Err(Error::custom("borrowed bytes not supported"))
        }

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Err(Error::custom("unit not supported"))
        }

        fn visit_none(self) -> Result<Self::Value, Error> {
            Err(Error::custom("none not supported"))
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Error> {
            Err(Error::custom("some not supported"))
        }
    }

    let content = Content::Str("test_str");
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<Error>,
    };
    let result = deserializer.deserialize_string(MockVisitor { value: None }).unwrap();
    assert_eq!(result, Some("test_str".to_string()));
}

#[test]
fn test_deserialize_string_with_byte_buf_content() {
    use crate::de::Visitor;
    use crate::de::value::Content;
    use crate::de::Error;
    
    struct MockVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<Vec<u8>>;

        fn visit_string(self, _value: String) -> Result<Self::Value, Error> {
            Err(Error::custom("string not supported"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("borrowed str not supported"))
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, Error> {
            Err(Error::custom("borrowed bytes not supported"))
        }

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Err(Error::custom("unit not supported"))
        }

        fn visit_none(self) -> Result<Self::Value, Error> {
            Err(Error::custom("none not supported"))
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Error> {
            Err(Error::custom("some not supported"))
        }
    }

    let content = Content::ByteBuf(vec![104, 101, 108, 108, 111]); // "hello" in bytes
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<Error>,
    };
    let result = deserializer.deserialize_string(MockVisitor { value: None }).unwrap();
    assert_eq!(result, Some(vec![104, 101, 108, 108, 111]));
}

#[test]
fn test_deserialize_string_with_bytes_content() {
    use crate::de::Visitor;
    use crate::de::value::Content;
    use crate::de::Error;
    
    struct MockVisitor {
        value: Option<&'static [u8]>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<&'static [u8]>;

        fn visit_string(self, _value: String) -> Result<Self::Value, Error> {
            Err(Error::custom("string not supported"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, Error> {
            Err(Error::custom("borrowed str not supported"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, Error> {
            Err(Error::custom("byte buf not supported"))
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Err(Error::custom("unit not supported"))
        }

        fn visit_none(self) -> Result<Self::Value, Error> {
            Err(Error::custom("none not supported"))
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value, Error> {
            Err(Error::custom("some not supported"))
        }
    }

    let content = Content::Bytes(b"hello"); // "hello" in bytes
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData::<Error>,
    };
    let result = deserializer.deserialize_string(MockVisitor { value: None }).unwrap();
    assert_eq!(result, Some(b"hello"));
}

