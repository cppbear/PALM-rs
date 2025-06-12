// Answer 0

#[test]
fn test_deserialize_float_f32() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value as f32)
        }
        
        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value as f32)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("unit unexpected"))
        }

        fn visit_string<E>(self, _: &str) -> Result<Self::Value, E> {
            Err(E::custom("string unexpected"))
        }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let result = deserializer.deserialize_float(TestVisitor);
    assert_eq!(result, Ok(3.14));
}

#[test]
fn test_deserialize_float_f64() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = f64;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            Ok(value as f64)
        }
        
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(E::custom("unit unexpected"))
        }

        fn visit_string<E>(self, _: &str) -> Result<Self::Value, E> {
            Err(E::custom("string unexpected"))
        }
    }

    let content = Content::F64(2.718);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let result = deserializer.deserialize_float(TestVisitor);
    assert_eq!(result, Ok(2.718));
}

#[test]
#[should_panic]
fn test_deserialize_float_invalid_type() {
    struct TestVisitor;

    impl Visitor<'_> for TestVisitor {
        type Value = ();

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            unreachable!()
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            unreachable!()
        }
        
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_string<E>(self, _: &str) -> Result<Self::Value, E> {
            unreachable!()
        }
    }

    let content = Content::Unit;
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let _result = deserializer.deserialize_float(TestVisitor);
}

