// Answer 0

#[test]
fn test_deserialize_bytes_string() {
    struct VisitorImpl {
        value: Option<String>,
    }

    impl<'de> serde::de::Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let result = String::from_utf8_lossy(value);
            Ok(result.to_string())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let result = String::from_utf8_lossy(value);
            Ok(result.to_string())
        }
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            unreachable!()
        }
    }

    struct ContentWrapper {
        content: Content,
    }

    impl ContentWrapper {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error
        where
            V: serde::de::Visitor<'de>,
        {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match &self.content {
                Content::String(ref v) => visitor.visit_str(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        String(String),
    }

    let content = Content::String("test".to_string());
    let wrapper = ContentWrapper { content };

    let visitor = VisitorImpl { value: None };
    let result = wrapper.deserialize_bytes(visitor).unwrap();
    assert_eq!(result, "test");
}

#[test]
fn test_deserialize_bytes_borrowed_str() {
    struct VisitorImpl {
        value: Option<String>,
    }

    impl<'de> serde::de::Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }
        
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let result = String::from_utf8_lossy(value);
            Ok(result.to_string())
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let result = String::from_utf8_lossy(value);
            Ok(result.to_string())
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            unreachable!()
        }
    }

    struct ContentWrapper {
        content: Content,
    }

    impl ContentWrapper {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error
        where
            V: serde::de::Visitor<'de>,
        {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match &self.content {
                Content::Str(v) => visitor.visit_borrowed_str(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        Str(&'static str),
    }

    let content = Content::Str("borrowed");
    let wrapper = ContentWrapper { content };

    let visitor = VisitorImpl { value: None };
    let result = wrapper.deserialize_bytes(visitor).unwrap();
    assert_eq!(result, "borrowed");
}

