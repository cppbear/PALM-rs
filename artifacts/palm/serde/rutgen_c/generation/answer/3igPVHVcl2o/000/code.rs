// Answer 0

#[test]
fn test_deserialize_str_with_string() {
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected bytes"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected borrowed bytes"))
        }
    }

    let content = Content::String("hello".to_owned());
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData::<()>,
    };
    let result: String = deserializer.deserialize_str(VisitorImpl).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_deserialize_str_with_str() {
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected bytes"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected borrowed bytes"))
        }
    }

    let content = Content::Str("world");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData::<()>,
    };
    let result: String = deserializer.deserialize_str(VisitorImpl).unwrap();
    assert_eq!(result, "world");
}

#[test]
fn test_deserialize_str_with_byte_buf() {
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Vec<u8>;

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            Err(E::custom("unexpected str"))
        }

        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            Err(E::custom("unexpected borrowed str"))
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }
    }

    let content = Content::ByteBuf(vec![1, 2, 3]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData::<()>,
    };
    let result: Vec<u8> = deserializer.deserialize_str(VisitorImpl).unwrap();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_deserialize_str_with_bytes() {
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Vec<u8>;

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            Err(E::custom("unexpected str"))
        }

        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            Err(E::custom("unexpected borrowed str"))
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected bytes"))
        }

        fn visit_borrowed_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }
    }

    let content = Content::Bytes(&[4, 5, 6][..]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData::<()>,
    };
    let result: Vec<u8> = deserializer.deserialize_str(VisitorImpl).unwrap();
    assert_eq!(result, vec![4, 5, 6]);
}

#[test]
#[should_panic]
fn test_deserialize_str_with_invalid_type() {
    struct VisitorImpl;
    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = String;

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            Ok("invalid".to_owned())
        }

        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            Ok("invalid".to_owned())
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected bytes"))
        }

        fn visit_borrowed_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Err(E::custom("unexpected borrowed bytes"))
        }
    }

    let content = Content::I32(42); // Invalid type for deserialization to string
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData::<()>,
    };
    let _result: String = deserializer.deserialize_str(VisitorImpl).unwrap();
}

