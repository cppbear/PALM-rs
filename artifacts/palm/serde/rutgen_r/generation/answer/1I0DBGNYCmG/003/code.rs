// Answer 0

#[derive(Debug)]
enum Content {
    F32(f32),
    F64(f64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

struct De<'de> {
    content: &'de Content,
}

impl<'de> De<'de> {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::F32(v) => visitor.visit_f32(v),
            Content::F64(v) => visitor.visit_f64(v),
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(v) => visitor.visit_i8(v),
            Content::I16(v) => visitor.visit_i16(v),
            Content::I32(v) => visitor.visit_i32(v),
            Content::I64(v) => visitor.visit_i64(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;
    fn visit_f32(self, v: f32) -> Result<Self::Value, String>;
    fn visit_f64(self, v: f64) -> Result<Self::Value, String>;
    fn visit_u8(self, v: u8) -> Result<Self::Value, String>;
    fn visit_u16(self, v: u16) -> Result<Self::Value, String>;
    fn visit_u32(self, v: u32) -> Result<Self::Value, String>;
    fn visit_u64(self, v: u64) -> Result<Self::Value, String>;
    fn visit_i8(self, v: i8) -> Result<Self::Value, String>;
    fn visit_i16(self, v: i16) -> Result<Self::Value, String>;
    fn visit_i32(self, v: i32) -> Result<Self::Value, String>;
    fn visit_i64(self, v: i64) -> Result<Self::Value, String>;
}

struct TestVisitor {
    result: f32,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = f32;

    fn visit_f32(self, v: f32) -> Result<Self::Value, String> {
        Ok(v)
    }

    fn visit_f64(self, _v: f64) -> Result<Self::Value, String> {
        Err("visit_f64 not supported".to_string())
    }

    // Implement other methods to return an error for unsupported types
    fn visit_u8(self, _v: u8) -> Result<Self::Value, String> { Err("visit_u8 not supported".to_string()) }
    fn visit_u16(self, _v: u16) -> Result<Self::Value, String> { Err("visit_u16 not supported".to_string()) }
    fn visit_u32(self, _v: u32) -> Result<Self::Value, String> { Err("visit_u32 not supported".to_string()) }
    fn visit_u64(self, _v: u64) -> Result<Self::Value, String> { Err("visit_u64 not supported".to_string()) }
    fn visit_i8(self, _v: i8) -> Result<Self::Value, String> { Err("visit_i8 not supported".to_string()) }
    fn visit_i16(self, _v: i16) -> Result<Self::Value, String> { Err("visit_i16 not supported".to_string()) }
    fn visit_i32(self, _v: i32) -> Result<Self::Value, String> { Err("visit_i32 not supported".to_string()) }
    fn visit_i64(self, _v: i64) -> Result<Self::Value, String> { Err("visit_i64 not supported".to_string()) }
}

#[test]
fn test_deserialize_float_f32() {
    let content = Content::F32(3.14);
    let de = De { content: &content };
    let visitor = TestVisitor { result: 0.0 };
    let result = de.deserialize_float(visitor);
    assert_eq!(result, Ok(3.14));
}

#[test]
fn test_deserialize_float_invalid_type() {
    let content = Content::I32(42);
    let de = De { content: &content };
    let visitor = TestVisitor { result: 0.0 };
    let result = de.deserialize_float(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

