// Answer 0

#[test]
fn test_deserialize_integer_u64() {
    struct TestVisitor {
        value: Option<u64>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u64>;

        fn visit_u64(self, value: u64) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(value))
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }
    }

    let content = Content::U64(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_integer(visitor).unwrap();
    assert_eq!(result, Some(42));
}

#[test]
fn test_deserialize_integer_unexpected() {
    struct TestVisitor {
        value: Option<u64>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u64>;

        fn visit_u64(self, _: u64) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected something else"))
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("expected u64"))
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_integer(visitor);
    assert!(result.is_err());
}

