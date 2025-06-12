// Answer 0

#[test]
fn test_deserialize_float_with_u32_content() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u32;

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }
        
        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_u32(self, value: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value)
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(0)
        }
    }

    let content = Content::U32(42);
    let content_deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = content_deserializer.deserialize_float(TestVisitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_float_with_invalid_content() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_f32(self, _: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_u8(self, _: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_u16(self, _: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_u32(self, _: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_i8(self, _: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_i16(self, _: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_i32(self, _: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    let content = Content::String(String::from("not a number"));
    let content_deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let result = content_deserializer.deserialize_float(TestVisitor);
    assert!(result.is_err());
}

