// Answer 0

#[derive(Debug)]
enum Content<'de> {
    Bytes(&'de [u8]),
    ByteBuf(Vec<u8>),
    String(String),
    Str(&'de str),
    Seq(Vec<Content<'de>>),
}

#[derive(Debug)]
struct TestDeserializer<'de> {
    content: Content<'de>,
}

impl<'de> TestDeserializer<'de> {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
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

    fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, String> {
        Ok("visited borrowed bytes".to_string())
    }

    fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, String> {
        Ok("visited borrowed str".to_string())
    }

    fn visit_string(self, _: String) -> Result<Self::Value, String> {
        Ok("visited string".to_string())
    }

    fn visit_byte_buf(self, _: Vec<u8>) -> Result<Self::Value, String> {
        Ok("visited byte buffer".to_string())
    }
}

#[test]
fn test_invalid_content() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = String;
    }

    let deserializer = TestDeserializer {
        content: Content::Seq(vec![]), // This example triggers the "invalid type"
    };

    let result = deserializer.deserialize_byte_buf(DummyVisitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

#[test]
fn test_invalid_content_string() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = String;
    }

    let deserializer = TestDeserializer {
        content: Content::String("Test".to_string()), // This triggers the "invalid type"
    };

    let result = deserializer.deserialize_byte_buf(DummyVisitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

#[test]
fn test_invalid_content_byte_buf() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = String;
    }

    let deserializer = TestDeserializer {
        content: Content::ByteBuf(vec![1, 2, 3]), // This triggers the "invalid type"
    };

    let result = deserializer.deserialize_byte_buf(DummyVisitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

