// Answer 0

#[derive(Debug)]
enum Content<'de> {
    Char(char),
    String(String),
    Str(&'de str),
}

struct Deserializer<'de> {
    content: &'de Content<'de>,
}

impl<'de> Deserializer<'de> {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match *self.content {
            Content::Char(v) => visitor.visit_char(v),
            Content::String(ref v) => visitor.visit_str(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_char(self, value: char) -> Result<Self::Value, String>;
    fn visit_str(self, value: &str) -> Result<Self::Value, String>;
    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, String>;
}

struct CharVisitor;

impl<'de> Visitor<'de> for CharVisitor {
    type Value = char;

    fn visit_char(self, value: char) -> Result<Self::Value, String> {
        Ok(value)
    }

    fn visit_str(self, _value: &str) -> Result<Self::Value, String> {
        Err("Cannot visit str as char".to_string())
    }

    fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, String> {
        Err("Cannot visit borrowed str as char".to_string())
    }
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = String;

    fn visit_char(self, _value: char) -> Result<Self::Value, String> {
        Err("Cannot visit char as str".to_string())
    }

    fn visit_str(self, value: &str) -> Result<Self::Value, String> {
        Ok(value.to_string())
    }

    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, String> {
        Ok(value.to_string())
    }
}

#[test]
fn test_deserialize_char_with_str_content() {
    let content = Content::Str("test");
    let deserializer = Deserializer { content: &content };
    let result: Result<char, String> = deserializer.deserialize_char(CharVisitor);
    
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Cannot visit borrowed str as char".to_string()));
}

#[test]
fn test_deserialize_char_with_string_content() {
    let content = Content::String("test".to_string());
    let deserializer = Deserializer { content: &content };
    let result: Result<char, String> = deserializer.deserialize_char(CharVisitor);

    assert!(result.is_err());
    assert_eq!(result.err(), Some("Cannot visit str as char".to_string()));
}

#[test]
fn test_deserialize_str_with_str_content() {
    let content = Content::Str("test");
    let deserializer = Deserializer { content: &content };
    let result: Result<String, String> = deserializer.deserialize_char(StrVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test".to_string());
}

#[test]
fn test_deserialize_str_with_string_content() {
    let content = Content::String("test".to_string());
    let deserializer = Deserializer { content: &content };
    let result: Result<String, String> = deserializer.deserialize_char(StrVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test".to_string());
}

