// Answer 0

#[test]
fn test_deserialize_integer_u32() {
    struct TestVisitor {
        value: Option<u32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u32;

        fn visit_u8(self, value: u8) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got u8: {}", value)))
        }

        fn visit_u16(self, value: u16) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got u16: {}", value)))
        }

        fn visit_u32(self, value: u32) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u64(self, value: u64) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got u64: {}", value)))
        }

        fn visit_i8(self, value: i8) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got i8: {}", value)))
        }

        fn visit_i16(self, value: i16) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got i16: {}", value)))
        }

        fn visit_i32(self, value: i32) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got i32: {}", value)))
        }

        fn visit_i64(self, value: i64) -> Result<Self::Value, E> {
            Err(E::custom(format!("Expected u32 but got i64: {}", value)))
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::U32(42),
        err: PhantomData,
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_integer(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_integer_invalid_type() {
    struct TestVisitor {
        value: Option<u32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u32;

        fn visit_u8(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("This is not a u32"))
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::String("not an integer".to_string()),
        err: PhantomData,
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_integer(visitor);
    assert!(result.is_err());
}

