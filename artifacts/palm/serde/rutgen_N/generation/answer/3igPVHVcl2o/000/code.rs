// Answer 0

#[test]
fn test_deserialize_str_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(String::from_utf8_lossy(value).to_string())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(String::from_utf8_lossy(value).to_string())
        }
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> String {
            "Invalid type".to_string()
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, String>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(ref v) => visitor.visit_bytes(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        String(String),
        Str(&'static str),
        ByteBuf(Vec<u8>),
        Bytes(&'static [u8]),
        Other,
    }

    let deserializer = TestDeserializer {
        content: Content::String("test".to_string()),
    };
    
    let result: Result<String, String> = deserializer.deserialize_str(TestVisitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_str_borrowed_str() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(String::from_utf8_lossy(value).to_string())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(String::from_utf8_lossy(value).to_string())
        }
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> String {
            "Invalid type".to_string()
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, String>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(ref v) => visitor.visit_bytes(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        String(String),
        Str(&'static str),
        ByteBuf(Vec<u8>),
        Bytes(&'static [u8]),
        Other,
    }

    let deserializer = TestDeserializer {
        content: Content::Str("borrowed"),
    };

    let result: Result<String, String> = deserializer.deserialize_str(TestVisitor);
    assert_eq!(result.unwrap(), "borrowed");
}

#[test]
fn test_deserialize_str_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(String::from_utf8_lossy(value).to_string())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(String::from_utf8_lossy(value).to_string())
        }
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> String {
            "Invalid type".to_string()
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, String>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(ref v) => visitor.visit_bytes(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        String(String),
        Str(&'static str),
        ByteBuf(Vec<u8>),
        Bytes(&'static [u8]),
        Other,
    }

    let deserializer = TestDeserializer {
        content: Content::Other,
    };
    
    let result: Result<String, String> = deserializer.deserialize_str(TestVisitor);
    assert_eq!(result.unwrap_err(), "Invalid type");
}

