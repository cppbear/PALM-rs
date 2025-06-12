// Answer 0

#[derive(Debug)]
enum Content<'de> {
    String(String),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),
    Invalid,
}

struct Deserializer<'de> {
    content: Content<'de>,
}

impl<'de> Deserializer<'de> {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::String(v) => visitor.visit_string(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(v) => visitor.visit_byte_buf(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;
    fn visit_string(self, v: String) -> Result<Self::Value, String>;
    fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value, String>;
    fn visit_byte_buf(self, v: Vec<u8>) -> Result<Self::Value, String>;
    fn visit_borrowed_bytes(self, v: &'de [u8]) -> Result<Self::Value, String>;
}

struct TestVisitor;

impl<'de> Visitor<'de> for TestVisitor {
    type Value = String;

    fn visit_string(self, _v: String) -> Result<Self::Value, String> {
        Ok("visited string".to_string())
    }

    fn visit_borrowed_str(self, _v: &'de str) -> Result<Self::Value, String> {
        Ok("visited borrowed str".to_string())
    }

    fn visit_byte_buf(self, _v: Vec<u8>) -> Result<Self::Value, String> {
        Ok("visited byte buf".to_string())
    }

    fn visit_borrowed_bytes(self, _v: &'de [u8]) -> Result<Self::Value, String> {
        Ok("visited borrowed bytes".to_string())
    }
}

#[test]
fn test_deserialize_invalid_type() {
    let deserializer = Deserializer {
        content: Content::Invalid,
    };
    let visitor = TestVisitor;

    let result = deserializer.deserialize_string(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

