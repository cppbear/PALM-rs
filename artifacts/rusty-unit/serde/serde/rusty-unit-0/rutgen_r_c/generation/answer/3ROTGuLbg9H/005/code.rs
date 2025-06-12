// Answer 0

#[test]
fn test_deserialize_integer_i8() {
    use crate::de::Visitor;
    use crate::de::Content;

    struct TestVisitor {
        value: Option<i8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<i8>;

        fn visit_i8(self, value: i8) -> Result<Self::Value, crate::de::Error> {
            Ok(Some(value))
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, crate::de::Error> {
            Err(crate::de::Error::custom("u8 is not expected"))
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, crate::de::Error> {
            Err(crate::de::Error::custom("i16 is not expected"))
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, crate::de::Error> {
            Err(crate::de::Error::custom("i32 is not expected"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, crate::de::Error> {
            Err(crate::de::Error::custom("i64 is not expected"))
        }

        // Additional visitor methods can be implemented as needed.
    }

    let content = Content::I8(-5);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_integer(visitor).unwrap();
    assert_eq!(result, Some(-5));
}

#[test]
fn test_deserialize_integer_invalid_type() {
    use crate::de::Visitor;
    use crate::de::Content;

    struct TestVisitor {
        value: Option<i8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<i8>;

        fn visit_i8(self, value: i8) -> Result<Self::Value, crate::de::Error> {
            Ok(Some(value))
        }

        // Only i8 visit method implemented
    }

    let content = Content::U8(10);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_integer(visitor);
    assert!(result.is_err());
}

