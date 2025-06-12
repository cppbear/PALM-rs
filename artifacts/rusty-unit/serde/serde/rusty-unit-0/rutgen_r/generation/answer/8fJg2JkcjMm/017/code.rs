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
    Map(Vec<(Content<'de>, Content<'de>)>),
}

struct ContentDeserializer<'de> {
    content: &'de Content<'de>,
}

impl<'de> ContentDeserializer<'de> {
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, ()>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::I8(v) => visitor.visit_i8(v),
            _ => Err(()),
        }
    }
}

trait Visitor<'de> {
    type Value;
    fn visit_i8(self, v: i8) -> Result<Self::Value, ()>;
}

struct TestVisitor {
    value: Option<i8>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = i8;
    
    fn visit_i8(self, v: i8) -> Result<Self::Value, ()> {
        Ok(v)
    }
}

#[test]
fn test_deserialize_i8() {
    let content = Content::I8(42);
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    
    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic]
fn test_deserialize_invalid_content() {
    let content = Content::Bool(true);
    let deserializer = ContentDeserializer { content: &content };
    let visitor = TestVisitor { value: None };
    
    deserializer.deserialize_any(visitor).unwrap();
}

