// Answer 0

#[test]
fn test_deserialize_bytes_with_string() {
    struct VisitorMock {
        pub visited_str: Option<String>,
        pub visited_bytes: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = &'de str;

        fn visit_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            self.visited_str = Some(value.to_string());
            Ok(value)
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected visit_bytes call"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected visit_borrowed_bytes call"))
        }
    }

    let content = Content::Str("test_string");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let mut visitor = VisitorMock {
        visited_str: None,
        visited_bytes: None,
    };

    let result = deserializer.deserialize_bytes(visitor);
    assert_eq!(result.unwrap(), "test_string");
    assert_eq!(visitor.visited_str.unwrap(), "test_string");
}

#[test]
fn test_deserialize_bytes_with_borrowed_str() {
    struct VisitorMock {
        pub visited_str: Option<String>,
        pub visited_bytes: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = &'de str;

        fn visit_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected visit_str call"))
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            self.visited_str = Some(value.to_string());
            Ok(value)
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected visit_bytes call"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("Unexpected visit_borrowed_bytes call"))
        }
    }

    let content = Content::Str("borrowed_string");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let mut visitor = VisitorMock {
        visited_str: None,
        visited_bytes: None,
    };

    let result = deserializer.deserialize_bytes(visitor);
    assert_eq!(result.unwrap(), "borrowed_string");
    assert_eq!(visitor.visited_str.unwrap(), "borrowed_string");
}

