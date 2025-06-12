// Answer 0

#[test]
fn test_deserialize_integer_u8() {
    struct Visitor {
        value: Option<u8>,
    }

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Option<u8>;

        fn visit_u8(self, value: u8) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an unsigned 8-bit integer")
        }
    }

    let content = Content::U8(42);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let result = deserializer.deserialize_integer(Visitor { value: None });
    assert_eq!(result.unwrap(), Some(42));
}

#[test]
fn test_deserialize_integer_i32() {
    struct Visitor {
        value: Option<i32>,
    }

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Option<i32>;

        fn visit_i32(self, value: i32) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a signed 32-bit integer")
        }
    }

    let content = Content::I32(-17);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let result = deserializer.deserialize_integer(Visitor { value: None });
    assert_eq!(result.unwrap(), Some(-17));
}

#[test]
fn test_deserialize_integer_invalid() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer")
        }
    }

    let content = Content::String("not an integer".to_string());
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    let result = deserializer.deserialize_integer(MockVisitor);
    assert!(result.is_err());
}

