// Answer 0

#[test]
fn test_deserialize_float_f32() {
    struct TestVisitor {
        value: Option<f32>,
    }

    impl Visitor<'_> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got f64"))
        }

        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u8"))
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u16"))
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u32"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u64"))
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i8"))
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i16"))
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i32"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i64"))
        }
    }

    let content = Content::F32(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
  
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_float(visitor);
  
    assert_eq!(result.unwrap(), 3.14);
}

#[test]
fn test_deserialize_float_f64() {
    struct TestVisitor {
        value: Option<f32>,
    }

    impl Visitor<'_> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got f32"))
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got f64"))
        }
      
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u8"))
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u16"))
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u32"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u64"))
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i8"))
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i16"))
        }

        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i32"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i64"))
        }
    }

    let content = Content::F64(3.14);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
  
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_float(visitor);
  
    assert!(result.is_err());
}

#[test]
fn test_deserialize_float_i32() {
    struct TestVisitor {
        value: Option<f32>,
    }

    impl Visitor<'_> for TestVisitor {
        type Value = f32;

        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got f32"))
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got f64"))
        }
      
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u8"))
        }

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u16"))
        }

        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u32"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got u64"))
        }

        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i8"))
        }

        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i16"))
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i32"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(E::custom("Expected f32, but got i64"))
        }
    }

    let content = Content::I32(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_float(visitor);
    
    assert!(result.is_err());
}

