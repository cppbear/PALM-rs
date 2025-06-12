// Answer 0

#[derive(Debug)]
struct Content {
    value: i8,
}

impl Content {
    fn I8(v: i8) -> Self {
        Content { value: v }
    }
}

#[derive(Debug)]
struct Serde;

impl Serde {
    fn serialize_i8(self, v: i8) -> Result<Content, &'static str> {
        Ok(Content::I8(v))
    }
}

#[test]
fn test_serialize_i8() {
    let serde_instance = Serde;
    let result = serde_instance.serialize_i8(42);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, 42);
    }
}

#[test]
fn test_serialize_i8_negative() {
    let serde_instance = Serde;
    let result = serde_instance.serialize_i8(-5);
    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.value, -5);
    }
}

