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
    content: Content<'de>,
}

impl<'de> ContentDeserializer<'de> {
    fn new(content: Content<'de>) -> Self {
        ContentDeserializer { content }
    }
}

trait Visitor<'de> {
    type Value;
    type Error;

    fn visit_bool(self, v: bool) -> Result<Self::Value, Self::Error>;
    fn visit_some(self, value: ContentDeserializer<'de>) -> Result<Self::Value, Self::Error>;
}

struct TestVisitor {
    value: Option<Content<'static>>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = ();
    type Error = ();

    fn visit_bool(self, v: bool) -> Result<Self::Value, Self::Error> {
        self.value = Some(Content::Bool(v));
        Ok(())
    }
    
    fn visit_some(self, value: ContentDeserializer<'de>) -> Result<Self::Value, Self::Error> {
        self.value = Some(Content::Some(Box::new(value.content)));
        Ok(())
    }
}

struct Deserializer {
    content: Content<'static>,
}

impl<'de> Deserializer {
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, V::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Some(v) => visitor.visit_some(ContentDeserializer::new(*v)),
            _ => panic!("Unexpected content type, expected Some"),
        }
    }
}

#[test]
fn test_deserialize_some() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    let deserializer = Deserializer { content };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_other_content() {
    let content = Content::Bool(true);
    let deserializer = Deserializer { content };
    let visitor = TestVisitor { value: None };
    deserializer.deserialize_any(visitor);
}

