// Answer 0

#[test]
fn test_deserialize_f64_valid() {
    struct TestVisitor {
        value: f64,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implement other required methods as no-ops
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(unimplemented!()) }
    }

    let content = Content::F64(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let result: f64 = deserializer.deserialize_f64(TestVisitor { value: 0.0 }).unwrap();
    assert_eq!(result, 3.14);
}

#[test]
fn test_deserialize_f64_invalid_type() {
    struct TestVisitor {
        value: f64,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }

        // Implement other required methods as no-ops
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> { Err(unimplemented!()) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Err(unimplemented!()) }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let result = deserializer.deserialize_f64(TestVisitor { value: 0.0 });
    assert!(result.is_err());
}

