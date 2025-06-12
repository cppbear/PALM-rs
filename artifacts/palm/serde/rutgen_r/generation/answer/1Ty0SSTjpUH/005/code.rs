// Answer 0

#[test]
fn test_deserialize_bytes_string_content() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Result<Vec<u8>, serde::de::Error>;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.as_bytes().to_vec()))
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.as_bytes().to_vec()))
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.to_vec()))
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.to_vec()))
        }
    }

    enum Content {
        String(String),
        Str(&'static str),
        ByteBuf(Vec<u8>),
        Bytes(&'static [u8]),
        Seq(Vec<u8>),
    }

    struct MockDeserializer {
        content: Box<Content>,
    }

    impl MockDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match *self.content {
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(ref v) => visitor.visit_bytes(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                Content::Seq(ref v) => visitor.visit_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let deserializer = MockDeserializer {
        content: Box::new(Content::Str("test")),
    };

    let visitor = MockVisitor;
    let result = deserializer.deserialize_bytes(visitor).unwrap().unwrap();
    assert_eq!(result, b"test".to_vec());
}

#[test]
fn test_deserialize_bytes_string_content_borrowed() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Result<Vec<u8>, serde::de::Error>;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.as_bytes().to_vec()))
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.as_bytes().to_vec()))
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.to_vec()))
        }

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Ok(value.to_vec()))
        }
    }

    enum Content {
        String(String),
        Str(&'static str),
        ByteBuf(Vec<u8>),
        Bytes(&'static [u8]),
        Seq(Vec<u8>),
    }

    struct MockDeserializer {
        content: Box<Content>,
    }

    impl MockDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match *self.content {
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                Content::ByteBuf(ref v) => visitor.visit_bytes(v),
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                Content::Seq(ref v) => visitor.visit_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let borrowed_string: &'static str = "borrowed";
    let deserializer = MockDeserializer {
        content: Box::new(Content::Str(borrowed_string)),
    };

    let visitor = MockVisitor;
    let result = deserializer.deserialize_bytes(visitor).unwrap().unwrap();
    assert_eq!(result, b"borrowed".to_vec());
}

