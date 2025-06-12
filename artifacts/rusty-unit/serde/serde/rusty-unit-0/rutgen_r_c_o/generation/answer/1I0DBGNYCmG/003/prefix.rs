// Answer 0

#[test]
fn test_deserialize_float_f32() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32(self, v: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v)
        }
        
        fn visit_f64(self, v: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u8(self, v: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u16(self, v: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u32(self, v: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u64(self, v: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i8(self, v: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i16(self, v: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i32(self, v: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i64(self, v: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
    }

    let float_value: f32 = 3.14;
    let content = Content::F32(float_value);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_f64() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32(self, v: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v)
        }
        
        fn visit_f64(self, v: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u8(self, v: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u16(self, v: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u32(self, v: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u64(self, v: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i8(self, v: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i16(self, v: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i32(self, v: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i64(self, v: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
    }

    let float_value: f64 = 2.71;
    let content = Content::F64(float_value);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_u8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32(self, v: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v)
        }
        
        fn visit_f64(self, v: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u8(self, v: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u16(self, v: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u32(self, v: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u64(self, v: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i8(self, v: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i16(self, v: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i32(self, v: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i64(self, v: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
    }

    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_i8() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = f32;

        fn visit_f32(self, v: f32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v)
        }
        
        fn visit_f64(self, v: f64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u8(self, v: u8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u16(self, v: u16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
        
        fn visit_u32(self, v: u32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_u64(self, v: u64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i8(self, v: i8) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i16(self, v: i16) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i32(self, v: i32) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }

        fn visit_i64(self, v: i64) -> Result<Self::Value, Box<dyn std::error::Error>> {
            Ok(v as f32)
        }
    }

    let content = Content::I8(-128);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_float(TestVisitor);
}

