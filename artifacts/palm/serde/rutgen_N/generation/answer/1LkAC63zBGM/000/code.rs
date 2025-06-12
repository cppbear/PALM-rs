// Answer 0

#[derive(Debug)]
struct Content {
    value: char,
}

impl Content {
    fn Char(v: char) -> Content {
        Content { value: v }
    }
}

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_char(self, v: char) -> Result<Content, &'static str> {
        Ok(Content::Char(v))
    }
}

#[test]
fn test_serialize_char() {
    let serializer = Serializer;
    let char_value = 'a';
    let result = serializer.serialize_char(char_value);

    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, char_value);
    }
}

#[test]
fn test_serialize_char_boundary() {
    let serializer = Serializer;
    let char_value = '\0'; // testing with null character
    let result = serializer.serialize_char(char_value);

    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, char_value);
    }
}

