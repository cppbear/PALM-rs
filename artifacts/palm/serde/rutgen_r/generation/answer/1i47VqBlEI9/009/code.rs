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

#[derive(Debug)]
struct Visitor;

impl<'de> Visitor<'de> {
    fn visit_f32(self, _value: f32) -> Result<u32, &'static str> {
        Err("Not a valid visit for F32")
    }
    
    fn visit_f64(self, _value: f64) -> Result<u32, &'static str> {
        Err("Not a valid visit for F64")
    }
    
    fn visit_u8(self, _value: u8) -> Result<u32, &'static str> {
        Err("Not a valid visit for U8")
    }
    
    fn visit_u16(self, _value: u16) -> Result<u32, &'static str> {
        Err("Not a valid visit for U16")
    }
    
    fn visit_u32(self, value: u32) -> Result<u32, &'static str> {
        Ok(value)
    }
    
    fn visit_u64(self, _value: u64) -> Result<u32, &'static str> {
        Err("Not a valid visit for U64")
    }
    
    fn visit_i8(self, _value: i8) -> Result<u32, &'static str> {
        Err("Not a valid visit for I8")
    }
    
    fn visit_i16(self, _value: i16) -> Result<u32, &'static str> {
        Err("Not a valid visit for I16")
    }
    
    fn visit_i32(self, _value: i32) -> Result<u32, &'static str> {
        Err("Not a valid visit for I32")
    }
    
    fn visit_i64(self, _value: i64) -> Result<u32, &'static str> {
        Err("Not a valid visit for I64")
    }
}

#[derive(Debug)]
struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::U32(v) => visitor.visit_u32(v),
            _ => Err("Invalid type for visitor"),
        }
    }
}

#[test]
fn test_deserialize_float_u32() {
    let deserializer = Deserializer {
        content: Content::U32(42),
    };
    let visitor = Visitor;
    
    let result = deserializer.deserialize_float(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic(expected = "Invalid type for visitor")]
fn test_deserialize_float_invalid() {
    let deserializer = Deserializer {
        content: Content::F32(3.14),
    };
    let visitor = Visitor;
    
    let _ = deserializer.deserialize_float(visitor);
}

