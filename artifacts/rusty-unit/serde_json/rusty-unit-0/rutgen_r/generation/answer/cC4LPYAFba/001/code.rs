// Answer 0

#[derive(Debug)]
struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, Error>
    where
        V: MapAccess<'de>,
    {
        unimplemented!()
    }
}

#[derive(Debug)]
enum Value {
    Object(std::collections::HashMap<String, Value>),
    String(String),
}

impl Value {
    fn invalid_type<V>(&self, _visitor: &V) -> Error {
        Error::custom("invalid type")
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Object(v) => v.deserialize_any(visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn custom(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

#[test]
fn test_deserialize_map_with_non_object_value() {
    let value = Value::String("not an object".to_string());
    let visitor = MockVisitor;

    let result = value.deserialize_map(visitor);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.message, "invalid type");
    }
}

