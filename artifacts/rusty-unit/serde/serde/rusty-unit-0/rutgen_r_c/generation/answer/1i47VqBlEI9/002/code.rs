// Answer 0

#[test]
fn test_deserialize_float_f64() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    let content = Content::F64(42.0);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let result: f64 = deserializer.deserialize_float(TestVisitor).unwrap();
    assert_eq!(result, 42.0);
}

#[test]
fn test_deserialize_float_f32() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let result: f32 = deserializer.deserialize_float(TestVisitor).unwrap();
    assert_eq!(result, 3.14);
}

#[test]
#[should_panic]
fn test_deserialize_float_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    let content = Content::Char('a');
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    deserializer.deserialize_float(TestVisitor).unwrap();
}

