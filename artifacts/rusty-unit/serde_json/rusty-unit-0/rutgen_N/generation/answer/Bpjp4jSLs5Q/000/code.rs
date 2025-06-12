// Answer 0

#[derive(Debug)]
struct DummyVisitor;

impl<'de> serde::de::Visitor<'de> for DummyVisitor {
    type Value = char;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a single character")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value.len() == 1 {
            Ok(value.chars().next().unwrap())
        } else {
            Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
        }
    }
}

#[test]
fn test_deserialize_char_valid() {
    let input = "\"a\"";
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = DummyVisitor;
    let result: Result<char, serde::de::Error> = deserializer.deserialize_char(visitor);
    assert_eq!(result, Ok('a'));
}

#[test]
fn test_deserialize_char_invalid() {
    let input = "\"ab\"";
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = DummyVisitor;
    let result: Result<char, serde::de::Error> = deserializer.deserialize_char(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_char_empty() {
    let input = "\"\"";
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = DummyVisitor;
    let result: Result<char, serde::de::Error> = deserializer.deserialize_char(visitor);
    assert!(result.is_err());
}

