// Answer 0

#[test]
fn test_deserialize_float_i16() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i16;

        fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        // Implement other visit methods as no-ops or error returns
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        // Other methods not necessary for this test can be ignored
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }
    }

    struct ContentData {
        content: Content,
    }

    impl ContentData {
        fn new_i16(value: i16) -> Self {
            ContentData {
                content: Content::I16(value),
            }
        }

        fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, serde::de::StdError>
        where
            V: Visitor<'static>,
        {
            match self.content {
                Content::I16(v) => visitor.visit_i16(v),
                _ => Err(serde::de::Error::custom("Invalid type")),
            }
        }
    }

    let content = ContentData::new_i16(42);
    let result = content.deserialize_float(TestVisitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_float_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i16;

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Should not reach here"))
        }

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("Expected i16"))
        }
    }

    struct ContentData {
        content: Content,
    }

    impl ContentData {
        fn new_f32(value: f32) -> Self {
            ContentData {
                content: Content::F32(value),
            }
        }

        fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, serde::de::StdError>
        where
            V: Visitor<'static>,
        {
            match self.content {
                Content::F32(v) => visitor.visit_f32(v),
                _ => Err(serde::de::Error::custom("Invalid type")),
            }
        }
    }

    let content = ContentData::new_f32(3.14);
    let result = content.deserialize_float(TestVisitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Expected i16");
}

