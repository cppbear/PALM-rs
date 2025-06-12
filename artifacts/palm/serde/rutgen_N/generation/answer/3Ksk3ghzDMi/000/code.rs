// Answer 0

#[test]
fn test_deserialize_integer_u8() {
    struct DummyVisitor {
        value: Option<u8>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = u8;

        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Required methods for other variants could be stubbed out
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not u16")) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not u32")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not u64")) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i8")) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i16")) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i32")) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i64")) }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::U8(v) => visitor.visit_u8(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let content = TestContent { content: Content::U8(42) };
    let visitor = DummyVisitor { value: None };
    let result = content.deserialize_integer(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_integer_invalid() {
    struct DummyVisitor {
        value: Option<u8>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = u8;

        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Other visit methods are stubs
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not u16")) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not u32")) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not u64")) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i8")) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i16")) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i32")) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(serde::de::Error::custom("Not i64")) }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::U8(v) => visitor.visit_u8(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let content = TestContent { content: Content::U16(42) };
    let visitor = DummyVisitor { value: None };
    let result = content.deserialize_integer(visitor);
    assert!(result.is_err());
}

