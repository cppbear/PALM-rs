// Answer 0

#[test]
fn test_deserialize_float_f32() {
    struct TestVisitor {
        value: Option<f32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            Ok(value)
        }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, found f64"))
        }
        // Implement other visit methods as needed...
    }

    let content = Content::F32(3.14);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let result: Result<f32, _> = deserializer.deserialize_float(visitor);
    assert_eq!(result, Ok(3.14));
}

#[test]
fn test_deserialize_float_f64() {
    struct TestVisitor {
        value: Option<f64>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f64, found f32"))
        }
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }
        // Implement other visit methods as needed...
    }

    let content = Content::F64(2.71);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let result: Result<f64, _> = deserializer.deserialize_float(visitor);
    assert_eq!(result, Ok(2.71));
}

#[test]
fn test_deserialize_float_invalid() {
    struct TestVisitor {
        value: Option<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(E::custom("Expected i32, found f32"))
        }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(E::custom("Expected i32, found f64"))
        }
        // Implement other visit methods as needed...
    }

    let content = Content::I32(42);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let result: Result<i32, _> = deserializer.deserialize_float(visitor);
    assert!(result.is_err());
}

