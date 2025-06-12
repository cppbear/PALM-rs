// Answer 0

#[derive(Debug)]
struct MockVisitor {
    value: Result<bool, &'static str>,
}

impl<'de> serde::de::Visitor<'de> for MockVisitor {
    type Value = bool;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean")
    }
}

#[derive(Debug)]
struct MockDeserializer {
    content: Box<Content>,
}

impl MockDeserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
        "Invalid type"
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: serde::de::Visitor<'de>,
    {
        match *self.content {
            Content::Bool(v) => visitor.visit_bool(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

#[derive(Debug)]
enum Content {
    Bool(bool),
    Other,
}

#[test]
fn test_deserialize_bool_valid() {
    let deserializer = MockDeserializer {
        content: Box::new(Content::Bool(true)),
    };
    let visitor = MockVisitor { value: Ok(true) };
    let result = deserializer.deserialize_bool(visitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_invalid_type() {
    let deserializer = MockDeserializer {
        content: Box::new(Content::Other),
    };
    let visitor = MockVisitor { value: Ok(true) };
    let result = deserializer.deserialize_bool(visitor);
    assert_eq!(result, Err("Invalid type"));
}

