// Answer 0

#[test]
fn test_deserialize_integer_u16() {
    struct TestVisitor {
        value: u16,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u16;

        fn visit_u16(self, value: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(value)
        }

        fn visit_u8(self, _value: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_u8 called".into())
        }

        fn visit_i8(self, _value: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_i8 called".into())
        }

        fn visit_i16(self, _value: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_i16 called".into())
        }

        fn visit_i32(self, _value: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_i32 called".into())
        }

        fn visit_i64(self, _value: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_i64 called".into())
        }

        fn visit_u32(self, _value: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_u32 called".into())
        }

        fn visit_u64(self, _value: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Err("visit_u64 called".into())
        }
    }

    let content = Content::U16(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };
    let visitor = TestVisitor { value: 0 };
    let result = deserializer.deserialize_integer(visitor);

    assert_eq!(result.unwrap(), 42);
}

