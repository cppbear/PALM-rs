// Answer 0

#[test]
fn test_deserialize_float_f64_positive() {
    struct TestVisitor;
    
    impl Visitor<'static> for TestVisitor {
        type Value = Result<f64, String>;

        fn visit_f32(self, _: f32) -> Self::Value {
            Err("visit_f32 not expected".to_string())
        }

        fn visit_f64(self, value: f64) -> Self::Value {
            Ok(value)
        }
        
        fn visit_u8(self, _: u8) -> Self::Value {
            Err("visit_u8 not expected".to_string())
        }

        fn visit_u16(self, _: u16) -> Self::Value {
            Err("visit_u16 not expected".to_string())
        }

        fn visit_u32(self, _: u32) -> Self::Value {
            Err("visit_u32 not expected".to_string())
        }

        fn visit_u64(self, _: u64) -> Self::Value {
            Err("visit_u64 not expected".to_string())
        }

        fn visit_i8(self, _: i8) -> Self::Value {
            Err("visit_i8 not expected".to_string())
        }

        fn visit_i16(self, _: i16) -> Self::Value {
            Err("visit_i16 not expected".to_string())
        }

        fn visit_i32(self, _: i32) -> Self::Value {
            Err("visit_i32 not expected".to_string())
        }

        fn visit_i64(self, _: i64) -> Self::Value {
            Err("visit_i64 not expected".to_string())
        }
    }

    let content = Content::F64(1.0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_f64_negative() {
    struct TestVisitor;

    impl Visitor<'static> for TestVisitor {
        type Value = Result<f64, String>;

        fn visit_f32(self, _: f32) -> Self::Value {
            Err("visit_f32 not expected".to_string())
        }

        fn visit_f64(self, value: f64) -> Self::Value {
            Ok(value)
        }
        
        fn visit_u8(self, _: u8) -> Self::Value {
            Err("visit_u8 not expected".to_string())
        }

        fn visit_u16(self, _: u16) -> Self::Value {
            Err("visit_u16 not expected".to_string())
        }

        fn visit_u32(self, _: u32) -> Self::Value {
            Err("visit_u32 not expected".to_string())
        }

        fn visit_u64(self, _: u64) -> Self::Value {
            Err("visit_u64 not expected".to_string())
        }

        fn visit_i8(self, _: i8) -> Self::Value {
            Err("visit_i8 not expected".to_string())
        }

        fn visit_i16(self, _: i16) -> Self::Value {
            Err("visit_i16 not expected".to_string())
        }

        fn visit_i32(self, _: i32) -> Self::Value {
            Err("visit_i32 not expected".to_string())
        }

        fn visit_i64(self, _: i64) -> Self::Value {
            Err("visit_i64 not expected".to_string())
        }
    }

    let content = Content::F64(-1.0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_f64_max() {
    struct TestVisitor;

    impl Visitor<'static> for TestVisitor {
        type Value = Result<f64, String>;

        fn visit_f32(self, _: f32) -> Self::Value {
            Err("visit_f32 not expected".to_string())
        }

        fn visit_f64(self, value: f64) -> Self::Value {
            Ok(value)
        }
        
        fn visit_u8(self, _: u8) -> Self::Value {
            Err("visit_u8 not expected".to_string())
        }

        fn visit_u16(self, _: u16) -> Self::Value {
            Err("visit_u16 not expected".to_string())
        }

        fn visit_u32(self, _: u32) -> Self::Value {
            Err("visit_u32 not expected".to_string())
        }

        fn visit_u64(self, _: u64) -> Self::Value {
            Err("visit_u64 not expected".to_string())
        }

        fn visit_i8(self, _: i8) -> Self::Value {
            Err("visit_i8 not expected".to_string())
        }

        fn visit_i16(self, _: i16) -> Self::Value {
            Err("visit_i16 not expected".to_string())
        }

        fn visit_i32(self, _: i32) -> Self::Value {
            Err("visit_i32 not expected".to_string())
        }

        fn visit_i64(self, _: i64) -> Self::Value {
            Err("visit_i64 not expected".to_string())
        }
    }

    let content = Content::F64(1.7976931348623157E+308);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

#[test]
fn test_deserialize_float_f64_min() {
    struct TestVisitor;

    impl Visitor<'static> for TestVisitor {
        type Value = Result<f64, String>;

        fn visit_f32(self, _: f32) -> Self::Value {
            Err("visit_f32 not expected".to_string())
        }

        fn visit_f64(self, value: f64) -> Self::Value {
            Ok(value)
        }
        
        fn visit_u8(self, _: u8) -> Self::Value {
            Err("visit_u8 not expected".to_string())
        }

        fn visit_u16(self, _: u16) -> Self::Value {
            Err("visit_u16 not expected".to_string())
        }

        fn visit_u32(self, _: u32) -> Self::Value {
            Err("visit_u32 not expected".to_string())
        }

        fn visit_u64(self, _: u64) -> Self::Value {
            Err("visit_u64 not expected".to_string())
        }

        fn visit_i8(self, _: i8) -> Self::Value {
            Err("visit_i8 not expected".to_string())
        }

        fn visit_i16(self, _: i16) -> Self::Value {
            Err("visit_i16 not expected".to_string())
        }

        fn visit_i32(self, _: i32) -> Self::Value {
            Err("visit_i32 not expected".to_string())
        }

        fn visit_i64(self, _: i64) -> Self::Value {
            Err("visit_i64 not expected".to_string())
        }
    }

    let content = Content::F64(-1.7976931348623157E+308);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let _ = deserializer.deserialize_float(TestVisitor);
}

