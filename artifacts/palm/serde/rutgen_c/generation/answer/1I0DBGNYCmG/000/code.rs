// Answer 0

#[test]
fn test_deserialize_float_f32() {
    struct TestVisitor;
    
    impl Visitor<'_> for TestVisitor {
        type Value = f32;
        
        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            Ok(value)
        }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(E::custom("Visited f64 instead of f32"))
        }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("Visited u8 instead of f32"))
        }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("Visited u16 instead of f32"))
        }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("Visited u32 instead of f32"))
        }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("Visited u64 instead of f32"))
        }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("Visited i8 instead of f32"))
        }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("Visited i16 instead of f32"))
        }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(E::custom("Visited i32 instead of f32"))
        }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Visited i64 instead of f32"))
        }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_float(TestVisitor);
    assert_eq!(result.unwrap(), 3.14);
}

#[test]
fn test_deserialize_float_f64() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = f64;
        
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(E::custom("Visited f32 instead of f64"))
        }
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("Visited u8 instead of f64"))
        }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("Visited u16 instead of f64"))
        }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("Visited u32 instead of f64"))
        }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("Visited u64 instead of f64"))
        }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("Visited i8 instead of f64"))
        }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("Visited i16 instead of f64"))
        }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(E::custom("Visited i32 instead of f64"))
        }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Visited i64 instead of f64"))
        }
    }

    let content = Content::F64(2.71);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_float(TestVisitor);
    assert_eq!(result.unwrap(), 2.71);
}

#[test]
fn test_deserialize_float_invalid_type() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = ();

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(E::custom("Visited f32 instead of expected type"))
        }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(E::custom("Visited f64 instead of expected type"))
        }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("Visited u8 instead of expected type"))
        }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("Visited u16 instead of expected type"))
        }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("Visited u32 instead of expected type"))
        }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("Visited u64 instead of expected type"))
        }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("Visited i8 instead of expected type"))
        }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("Visited i16 instead of expected type"))
        }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(E::custom("Visited i32 instead of expected type"))
        }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Visited i64 instead of expected type"))
        }
    }

    let content = Content::Char('a');
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_float(TestVisitor);
    assert!(result.is_err());
}

