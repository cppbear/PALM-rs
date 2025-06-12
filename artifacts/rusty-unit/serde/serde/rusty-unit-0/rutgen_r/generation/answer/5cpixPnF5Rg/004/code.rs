// Answer 0

#[derive(Debug)]
enum Content {
    Char(char),
    String(String),
    Str(&'static str),
    // other variants
}

struct TestDeserializer {
    content: Content,
}

impl TestDeserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
        "Invalid type"
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'de>,
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

struct TestVisitor {
    // additional fields if necessary
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = char;

    fn visit_char(self, value: char) -> Result<Self::Value, &'static str> {
        Ok(value)
    }

    fn visit_string(self, _value: String) -> Result<Self::Value, &'static str> {
        Err("Expected char, got string")
    }

    fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, &'static str> {
        Err("Expected char, got borrowed str")
    }
}

#[test]
fn test_deserialize_char_with_char_content() {
    let deserializer = TestDeserializer {
        content: Content::Char('a'),
    };
    let visitor = TestVisitor {};
    let result = deserializer.deserialize_char(visitor).unwrap();
    assert_eq!(result, 'a');
}

#[test]
fn test_deserialize_char_with_string_content() {
    let deserializer = TestDeserializer {
        content: Content::String("hello".to_string()),
    };
    let visitor = TestVisitor {};
    let result = deserializer.deserialize_char(visitor);
    assert_eq!(result, Err("Expected char, got string"));
}

#[test]
fn test_deserialize_char_with_borrowed_str_content() {
    let deserializer = TestDeserializer {
        content: Content::Str("world"),
    };
    let visitor = TestVisitor {};
    let result = deserializer.deserialize_char(visitor);
    assert_eq!(result, Err("Expected char, got borrowed str"));
}

