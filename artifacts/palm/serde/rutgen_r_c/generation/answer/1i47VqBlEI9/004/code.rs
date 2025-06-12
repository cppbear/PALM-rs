// Answer 0

#[test]
fn test_deserialize_float_i64() {
    struct TestVisitor {
        value: Option<i64>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<i64>;

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected f32".into())
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Unexpected f64".into())
        }

        fn visit_u8(self, value: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_u16(self, value: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_u32(self, value: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_u64(self, value: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_i8(self, value: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_i16(self, value: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_i32(self, value: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value as i64))
        }

        fn visit_i64(self, value: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(Some(value))
        }
    }

    let content = Content::I64(42);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_float(visitor).expect("Expected successful deserialization");

    assert_eq!(result, Some(42));
}

#[test]
fn test_deserialize_float_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("Should not be called".into())
        }
    }

    let content = Content::String(String::from("invalid"));
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = TestVisitor;
    let result = deserializer.deserialize_float(visitor);

    assert!(result.is_err(), "Expected error for invalid type");
}

