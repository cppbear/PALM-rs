// Answer 0

#[test]
fn test_deserialize_bytes_string() {
    struct VisitorImpl {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }

        fn visit_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed bytes"))
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, E>
        where
            V: SeqAccess<'de>,
        {
            Err(E::custom("Unexpected sequence"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let content = Content::String("test".to_string());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = VisitorImpl { value: None };
    let result = deserializer.deserialize_bytes(visitor).unwrap();
    assert_eq!(result, "test".to_string());
}

#[test]
fn test_deserialize_bytes_str() {
    struct VisitorImpl {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_string())
        }

        fn visit_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected bytes"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected borrowed bytes"))
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, E>
        where
            V: SeqAccess<'de>,
        {
            Err(E::custom("Unexpected sequence"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected unit"))
        }
    }

    let content = Content::Str("test_str");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = VisitorImpl { value: None };
    let result = deserializer.deserialize_bytes(visitor).unwrap();
    assert_eq!(result, "test_str".to_string());
}

