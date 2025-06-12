// Answer 0

#[test]
fn test_deserialize_byte_buf_with_borrowed_bytes() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_vec())
        }

        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("unexpected string"))
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("unexpected string"))
        }

        fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
    }

    enum Content {
        Bytes(&'static [u8]),
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _: &V) -> serde::de::Error {
            serde::de::Error::custom("invalid type")
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let deserializer = TestDeserializer {
        content: Content::Bytes(b"test_bytes"),
    };

    let result = deserializer.deserialize_byte_buf(TestVisitor);
    assert_eq!(result, Ok(b"test_bytes".to_vec()));
}

#[test]
fn test_deserialize_byte_buf_with_invalid_type() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("unexpected bytes"))
        }

        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("unexpected string"))
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("unexpected string"))
        }

        fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Err(E::custom("unexpected byte buffer"))
        }
    }

    enum Content {
        Str(&'static str),
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _: &V) -> serde::de::Error {
            serde::de::Error::custom("invalid type")
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Str(_) => visitor.visit_borrowed_str("invalid"),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let deserializer = TestDeserializer {
        content: Content::Str("invalid"),
    };

    let result = deserializer.deserialize_byte_buf(TestVisitor);
    assert!(result.is_err());
}

