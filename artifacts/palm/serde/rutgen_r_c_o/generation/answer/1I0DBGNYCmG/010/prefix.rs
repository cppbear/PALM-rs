// Answer 0

#[test]
fn test_deserialize_float_with_u16() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u16;

        fn visit_f32(self, _: f32) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_f32 not expected"))
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_f64 not expected"))
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u8 not expected"))
        }

        fn visit_u16(self, value: u16) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u32 not expected"))
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u64 not expected"))
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i8 not expected"))
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i16 not expected"))
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i32 not expected"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i64 not expected"))
        }
    }

    let content = Content::U16(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = TestVisitor;
    let _ = deserializer.deserialize_float(visitor);
}

#[test]
fn test_deserialize_float_with_u16_max() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u16;

        fn visit_f32(self, _: f32) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_f32 not expected"))
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_f64 not expected"))
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u8 not expected"))
        }

        fn visit_u16(self, value: u16) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u32 not expected"))
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u64 not expected"))
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i8 not expected"))
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i16 not expected"))
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i32 not expected"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i64 not expected"))
        }
    }

    let content = Content::U16(65535);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = TestVisitor;
    let _ = deserializer.deserialize_float(visitor);
}

