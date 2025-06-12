// Answer 0

#[test]
fn test_deserialize_integer_u16() {
    struct MockVisitor {
        value: Option<u16>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = u16;

        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
            self.value = Some(value);
            Ok(value)
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected u16"))
        }
    }

    struct ContentWrapper {
        content: Box<Content>,
    }

    enum Content {
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
    }

    let content = ContentWrapper {
        content: Box::new(Content::U16(42)),
    };

    let result = content.deserialize_integer(MockVisitor { value: None });

    assert_eq!(result, Ok(42));
}

