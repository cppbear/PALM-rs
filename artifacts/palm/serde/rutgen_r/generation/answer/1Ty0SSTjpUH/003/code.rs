// Answer 0

#[derive(Debug)]
enum Content<'de> {
    String(&'de str),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),
    Seq(Vec<Content<'de>>),
}

struct Deserializer<'de> {
    content: Box<Content<'de>>,
}

impl<'de> Deserializer<'de> {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::String(ref v) => visitor.visit_str(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::ByteBuf(ref v) => visitor.visit_bytes(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::Seq(ref v) => visit_content_seq_ref(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_str(self, value: &'de str) -> Result<Self::Value, String>;
    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, String>;
    fn visit_bytes(self, value: &[u8]) -> Result<Self::Value, String>;
    fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, String>;
}

struct TestVisitor {
    bytes_result: Vec<u8>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Vec<u8>;

    fn visit_str(self, _value: &'de str) -> Result<Self::Value, String> {
        Err("Unexpected string".to_owned())
    }

    fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, String> {
        Err("Unexpected borrowed string".to_owned())
    }

    fn visit_bytes(self, value: &[u8]) -> Result<Self::Value, String> {
        Ok(value.to_vec())
    }

    fn visit_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, String> {
        Ok(value.to_vec())
    }
}

fn visit_content_seq_ref<'de, V>(_content: &'de [Content<'de>], _visitor: V) -> Result<V::Value, String>
where
    V: Visitor<'de>,
{
    Err("Unexpected sequence".to_string())
}

#[test]
fn test_deserialize_bytes_with_borrowed_bytes() {
    let content = Content::Bytes(&[1, 2, 3]);
    let deserializer = Deserializer { content: Box::new(content) };
    let visitor = TestVisitor { bytes_result: Vec::new() };

    let result = deserializer.deserialize_bytes(visitor);
    assert_eq!(result, Ok(vec![1, 2, 3]));
}

#[test]
fn test_deserialize_bytes_with_invalid_type() {
    let content = Content::String("not bytes");
    let deserializer = Deserializer { content: Box::new(content) };
    let visitor = TestVisitor { bytes_result: Vec::new() };

    let result = deserializer.deserialize_bytes(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

