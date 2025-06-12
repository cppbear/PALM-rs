// Answer 0

#[derive(Debug)]
enum Content<'de> {
    String(&'de str),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),
    U8(u8),
    U64(u64),
}

struct MyDeserializer<'de> {
    content: Content<'de>,
}

impl<'de> MyDeserializer<'de> {
    fn invalid_type<V>(&self, _: &V) -> &'static str {
        "Invalid type"
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::String(v) => visitor.visit_string(v.into()),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(v) => visitor.visit_byte_buf(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::U8(v) => visitor.visit_u8(v),
            Content::U64(v) => visitor.visit_u64(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_string(self, value: String) -> Result<Self::Value, &'static str>;
    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, &'static str>;
    fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, &'static str>;
    fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, &'static str>;
    fn visit_u8(self, value: u8) -> Result<Self::Value, &'static str>;
    fn visit_u64(self, value: u64) -> Result<Self::Value, &'static str>;
}

struct TestVisitor;

impl<'de> Visitor<'de> for TestVisitor {
    type Value = &'de [u8];

    fn visit_string(self, value: String) -> Result<Self::Value, &'static str> {
        Ok(value.as_bytes())
    }
    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, &'static str> {
        Ok(value.as_bytes())
    }
    fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, &'static str> {
        Ok(&value)
    }
    fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, &'static str> {
        Ok(value)
    }
    fn visit_u8(self, _value: u8) -> Result<Self::Value, &'static str> {
        Err("Should not visit u8")
    }
    fn visit_u64(self, _value: u64) -> Result<Self::Value, &'static str> {
        Err("Should not visit u64")
    }
}

#[test]
fn test_deserialize_identifier_bytes() {
    let content = Content::Bytes(b"hello world");
    let deserializer = MyDeserializer { content };
    let visitor = TestVisitor;

    let result = deserializer.deserialize_identifier(visitor);
    assert_eq!(result.unwrap(), b"hello world");
}

#[test]
fn test_deserialize_identifier_borrowed_bytes() {
    let content = Content::Bytes(b"test");
    let deserializer = MyDeserializer { content };
    let visitor = TestVisitor;

    let result = deserializer.deserialize_identifier(visitor);
    assert_eq!(result.unwrap(), b"test");
}

