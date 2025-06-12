// Answer 0


#[derive(Debug)]
enum Content {
    Char(char),
    String(String),
    Str(&'static str),
    Other,
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
        "Invalid type"
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'static>,
    {
        match self.content {
            Content::Char(v) => visitor.visit_char(v),
            Content::String(v) => visitor.visit_string(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_char(self, value: char) -> Result<Self::Value, &'static str>;
    fn visit_string(self, value: String) -> Result<Self::Value, &'static str>;
    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, &'static str>;
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_char(self, _value: char) -> Result<Self::Value, &'static str> {
        Ok(())
    }

    fn visit_string(self, _value: String) -> Result<Self::Value, &'static str> {
        Ok(())
    }

    fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, &'static str> {
        Ok(())
    }
}

#[test]
fn test_deserialize_char_invalid_type() {
    let deserializer = Deserializer {
        content: Content::Other,
    };
    let visitor = MockVisitor;
    let result = deserializer.deserialize_char(visitor);
    assert_eq!(result, Err("Invalid type"));
}


