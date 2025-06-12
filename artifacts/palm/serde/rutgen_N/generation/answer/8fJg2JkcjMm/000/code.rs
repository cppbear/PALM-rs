// Answer 0

#[derive(Debug)]
enum Content<'de> {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),
    Unit,
    None,
    Some(Box<Content<'de>>),
    Newtype(Box<Content<'de>>),
    Seq(Vec<Content<'de>>),
    Map(std::collections::HashMap<String, Content<'de>>),
}

struct ContentDeserializer<'de> {
    content: &'de Content<'de>,
}

impl<'de> ContentDeserializer<'de> {
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::Bool(v) => visitor.visit_bool(v),
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(v) => visitor.visit_i8(v),
            Content::I16(v) => visitor.visit_i16(v),
            Content::I32(v) => visitor.visit_i32(v),
            Content::I64(v) => visitor.visit_i64(v),
            Content::F32(v) => visitor.visit_f32(v),
            Content::F64(v) => visitor.visit_f64(v),
            Content::Char(v) => visitor.visit_char(v),
            Content::String(ref v) => visitor.visit_str(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(ref v) => visitor.visit_bytes(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::Unit => visitor.visit_unit(),
            Content::None => visitor.visit_none(),
            Content::Some(ref v) => visitor.visit_some(ContentDeserializer { content: v }),
            Content::Newtype(ref v) => {
                visitor.visit_newtype_struct(ContentDeserializer { content: v })
            }
            Content::Seq(ref v) => visitor.visit_seq(ContentDeserializer { content: &v }),
            Content::Map(ref v) => visitor.visit_map(ContentDeserializer { content: &v }),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_bool(self, v: bool) -> Result<Self::Value, String>;
    fn visit_u8(self, v: u8) -> Result<Self::Value, String>;
    fn visit_u16(self, v: u16) -> Result<Self::Value, String>;
    fn visit_u32(self, v: u32) -> Result<Self::Value, String>;
    fn visit_u64(self, v: u64) -> Result<Self::Value, String>;
    fn visit_i8(self, v: i8) -> Result<Self::Value, String>;
    fn visit_i16(self, v: i16) -> Result<Self::Value, String>;
    fn visit_i32(self, v: i32) -> Result<Self::Value, String>;
    fn visit_i64(self, v: i64) -> Result<Self::Value, String>;
    fn visit_f32(self, v: f32) -> Result<Self::Value, String>;
    fn visit_f64(self, v: f64) -> Result<Self::Value, String>;
    fn visit_char(self, v: char) -> Result<Self::Value, String>;
    fn visit_str(self, v: &str) -> Result<Self::Value, String>;
    fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value, String>;
    fn visit_bytes(self, v: &[u8]) -> Result<Self::Value, String>;
    fn visit_borrowed_bytes(self, v: &'de [u8]) -> Result<Self::Value, String>;
    fn visit_unit(self) -> Result<Self::Value, String>;
    fn visit_none(self) -> Result<Self::Value, String>;
    fn visit_some(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String>;
    fn visit_newtype_struct(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String>;
    fn visit_seq(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String>;
    fn visit_map(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String>;
}

struct TestVisitor {
    value: Option<String>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = String;

    fn visit_bool(self, v: bool) -> Result<Self::Value, String> {
        Ok(format!("Bool: {}", v))
    }

    fn visit_u8(self, v: u8) -> Result<Self::Value, String> {
        Ok(format!("U8: {}", v))
    }

    fn visit_u16(self, v: u16) -> Result<Self::Value, String> {
        Ok(format!("U16: {}", v))
    }

    fn visit_u32(self, v: u32) -> Result<Self::Value, String> {
        Ok(format!("U32: {}", v))
    }

    fn visit_u64(self, v: u64) -> Result<Self::Value, String> {
        Ok(format!("U64: {}", v))
    }

    fn visit_i8(self, v: i8) -> Result<Self::Value, String> {
        Ok(format!("I8: {}", v))
    }

    fn visit_i16(self, v: i16) -> Result<Self::Value, String> {
        Ok(format!("I16: {}", v))
    }

    fn visit_i32(self, v: i32) -> Result<Self::Value, String> {
        Ok(format!("I32: {}", v))
    }

    fn visit_i64(self, v: i64) -> Result<Self::Value, String> {
        Ok(format!("I64: {}", v))
    }

    fn visit_f32(self, v: f32) -> Result<Self::Value, String> {
        Ok(format!("F32: {}", v))
    }

    fn visit_f64(self, v: f64) -> Result<Self::Value, String> {
        Ok(format!("F64: {}", v))
    }

    fn visit_char(self, v: char) -> Result<Self::Value, String> {
        Ok(format!("Char: {}", v))
    }

    fn visit_str(self, v: &str) -> Result<Self::Value, String> {
        Ok(format!("String: {}", v))
    }

    fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value, String> {
        Ok(format!("Borrowed Str: {}", v))
    }

    fn visit_bytes(self, v: &[u8]) -> Result<Self::Value, String> {
        Ok(format!("ByteBuf: {:?}", v))
    }

    fn visit_borrowed_bytes(self, v: &'de [u8]) -> Result<Self::Value, String> {
        Ok(format!("Borrowed Bytes: {:?}", v))
    }

    fn visit_unit(self) -> Result<Self::Value, String> {
        Ok("Unit".to_string())
    }

    fn visit_none(self) -> Result<Self::Value, String> {
        Ok("None".to_string())
    }

    fn visit_some(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String> {
        v.deserialize_any(self)
    }

    fn visit_newtype_struct(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String> {
        v.deserialize_any(self)
    }

    fn visit_seq(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String> {
        Ok("Seq".to_string())
    }

    fn visit_map(self, v: ContentDeserializer<'de>) -> Result<Self::Value, String> {
        Ok("Map".to_string())
    }
}

#[test]
fn test_deserialize_bool() {
    let content = Content::Bool(true);
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    assert_eq!(deserializer.deserialize_any(visitor).unwrap(), "Bool: true");
}

#[test]
fn test_deserialize_u8() {
    let content = Content::U8(255);
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    assert_eq!(deserializer.deserialize_any(visitor).unwrap(), "U8: 255");
}

#[test]
fn test_deserialize_str() {
    let content = Content::String("hello".to_string());
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    assert_eq!(deserializer.deserialize_any(visitor).unwrap(), "String: hello");
}

#[test]
fn test_deserialize_unit() {
    let content = Content::Unit;
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    assert_eq!(deserializer.deserialize_any(visitor).unwrap(), "Unit");
}

#[test]
fn test_deserialize_none() {
    let content = Content::None;
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    assert_eq!(deserializer.deserialize_any(visitor).unwrap(), "None");
}

