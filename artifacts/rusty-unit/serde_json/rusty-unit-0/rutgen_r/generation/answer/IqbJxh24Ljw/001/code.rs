// Answer 0

#[derive(Debug)]
struct MockVisitor;

impl<'de> serde::de::Visitor<'de> for MockVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_owned())
    }

    fn visit_null<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(E::custom("null is not a valid string"))
    }
}

#[test]
fn test_deserialize_string_valid() {
    let input = r#""valid string""#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = MockVisitor;
    let result: Result<String, _> = deserializer.deserialize_string(visitor);
    assert_eq!(result.unwrap(), "valid string");
}

#[test]
#[should_panic(expected = "null is not a valid string")]
fn test_deserialize_string_null() {
    let input = r#"null"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = MockVisitor;
    let _result: Result<String, _> = deserializer.deserialize_string(visitor).unwrap();
}

