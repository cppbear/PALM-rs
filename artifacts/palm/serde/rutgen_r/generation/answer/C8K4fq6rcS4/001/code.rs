// Answer 0

#[derive(Debug)]
struct Content {
    value: String,
}

impl Content {
    fn String(value: String) -> Self {
        Content { value }
    }
}

struct Serializer;

impl Serializer {
    fn serialize_str(self, value: &str) -> Result<Content, &'static str> {
        if value.is_empty() {
            return Err("Value cannot be empty");
        }
        Ok(Content::String(value.to_owned()))
    }
}

#[test]
fn test_serialize_str_valid_input() {
    let serializer = Serializer;
    let result = serializer.serialize_str("hello");
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, "hello");
    }
}

#[test]
fn test_serialize_str_empty_input() {
    let serializer = Serializer;
    let result = serializer.serialize_str("");
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Value cannot be empty");
}

#[test]
fn test_serialize_str_whitespace_input() {
    let serializer = Serializer;
    let result = serializer.serialize_str("   ");
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, "   ");
    }
}

#[test]
fn test_serialize_str_special_characters() {
    let serializer = Serializer;
    let result = serializer.serialize_str("!@#$%^&*()");
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, "!@#$%^&*()");
    }
}

