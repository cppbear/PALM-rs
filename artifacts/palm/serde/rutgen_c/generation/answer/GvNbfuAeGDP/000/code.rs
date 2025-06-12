// Answer 0

#[test]
fn test_deserialize_string_with_string() {
    struct DummyVisitor;
    
    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = String;

        fn visit_string(self, value: String) -> Result<Self::Value, serde::de::Error> {
            assert_eq!(value, "test_string");
            Ok(value)
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected byte buf"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow bytes"))
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected unit"))
        }
    }

    let content = Content::String("test_string".to_string());
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_string(DummyVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_string_with_str() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = &'de str;

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected string"))
        }

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::Error> {
            assert_eq!(value, "test_str");
            Ok(value)
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected byte buf"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow bytes"))
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected unit"))
        }
    }

    let content = Content::Str("test_str");
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_string(DummyVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_string_with_byte_buf() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Vec<u8>;

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected string"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow"))
        }

        fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            assert_eq!(value, b"test_bytes".to_vec());
            Ok(value)
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow bytes"))
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected unit"))
        }
    }

    let content = Content::ByteBuf(b"test_bytes".to_vec());
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_string(DummyVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_string_with_bytes() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = &'de [u8];

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected string"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected byte buf"))
        }

        fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            assert_eq!(value, b"test_bytes");
            Ok(value)
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected unit"))
        }
    }

    let content = Content::Bytes(b"test_bytes");
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_string(DummyVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_string_invalid_content() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();

        fn visit_string(self, _value: String) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected string"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow"))
        }

        fn visit_byte_buf(self, _value: Vec<u8>) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected byte buf"))
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected borrow bytes"))
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("unexpected unit"))
        }
    }

    let content = Content::None; // Using invalid content
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_string(DummyVisitor);
    assert!(result.is_err());
}

