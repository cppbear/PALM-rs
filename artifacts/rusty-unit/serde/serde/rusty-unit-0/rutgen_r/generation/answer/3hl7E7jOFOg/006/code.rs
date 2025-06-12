// Answer 0

#[derive(Debug)]
enum Content {
    String(String),
    Str(&'static str),
    ByteBuf(Vec<u8>),
    Bytes(&'static [u8]),
    Seq(Vec<Content>),
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::String(v) => visitor.visit_string(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(v) => visitor.visit_byte_buf(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::Seq(v) => visit_content_seq(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_string(self, value: String) -> Result<Self::Value, String>;
    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, String>;
    fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, String>;
    fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, String>;
}

#[derive(Debug)]
struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn visit_string(self, value: String) -> Result<Self::Value, String> {
        Ok(value)
    }

    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, String> {
        Ok(value.to_string())
    }

    fn visit_byte_buf(self, value: Vec<u8>) -> Result<Self::Value, String> {
        Ok(String::from_utf8(value).unwrap())
    }

    fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, String> {
        Ok(String::from_utf8(value.to_vec()).unwrap())
    }
}

#[test]
fn test_deserialize_string() {
    let content = Content::String("test string".to_string());
    let deserializer = Deserializer { content };
    
    let visitor = StringVisitor;
    let result = deserializer.deserialize_byte_buf(visitor).unwrap();
    
    assert_eq!(result, "test string");
}

#[test]
fn test_deserialize_borrowed_str() {
    let content = Content::Str("borrowed string");
    let deserializer = Deserializer { content };

    let visitor = StringVisitor;
    let result = deserializer.deserialize_byte_buf(visitor).unwrap();
    
    assert_eq!(result, "borrowed string");
}

