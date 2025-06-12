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
    Invalid,
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        String::from("Invalid type")
    }

    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'_>,
    {
        match self.content {
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
    
    fn visit_f32(self, value: f32) -> Result<Self::Value, String>;
    fn visit_f64(self, value: f64) -> Result<Self::Value, String>;
    fn visit_u8(self, value: u8) -> Result<Self::Value, String>;
    fn visit_u16(self, value: u16) -> Result<Self::Value, String>;
    fn visit_u32(self, value: u32) -> Result<Self::Value, String>;
    fn visit_u64(self, value: u64) -> Result<Self::Value, String>;
    fn visit_i8(self, value: i8) -> Result<Self::Value, String>;
    fn visit_i16(self, value: i16) -> Result<Self::Value, String>;
    fn visit_i32(self, value: i32) -> Result<Self::Value, String>;
    fn visit_i64(self, value: i64) -> Result<Self::Value, String>;
}

struct TestVisitor {
    value: Option<f64>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Option<f64>;
    
    fn visit_f32(self, _value: f32) -> Result<Self::Value, String> {
        Err(String::from("visit_f32 not implemented"))
    }
    
    fn visit_f64(self, value: f64) -> Result<Self::Value, String> {
        Ok(Some(value))
    }

    fn visit_u8(self, _value: u8) -> Result<Self::Value, String> {
        Err(String::from("visit_u8 not implemented"))
    }

    fn visit_u16(self, _value: u16) -> Result<Self::Value, String> {
        Err(String::from("visit_u16 not implemented"))
    }

    fn visit_u32(self, _value: u32) -> Result<Self::Value, String> {
        Err(String::from("visit_u32 not implemented"))
    }

    fn visit_u64(self, _value: u64) -> Result<Self::Value, String> {
        Err(String::from("visit_u64 not implemented"))
    }

    fn visit_i8(self, _value: i8) -> Result<Self::Value, String> {
        Err(String::from("visit_i8 not implemented"))
    }

    fn visit_i16(self, _value: i16) -> Result<Self::Value, String> {
        Err(String::from("visit_i16 not implemented"))
    }

    fn visit_i32(self, _value: i32) -> Result<Self::Value, String> {
        Err(String::from("visit_i32 not implemented"))
    }

    fn visit_i64(self, _value: i64) -> Result<Self::Value, String> {
        Err(String::from("visit_i64 not implemented"))
    }
}

#[test]
fn test_deserialize_float_f64() {
    let deserializer = Deserializer {
        content: Content::F64(3.14),
    };
    let visitor = TestVisitor { value: None };
    
    let result = deserializer.deserialize_float(visitor);
    assert_eq!(result.unwrap(), Some(3.14));
}

#[test]
fn test_deserialize_float_invalid_type() {
    let deserializer = Deserializer {
        content: Content::Invalid,
    };
    let visitor = TestVisitor { value: None };
    
    let result = deserializer.deserialize_float(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

