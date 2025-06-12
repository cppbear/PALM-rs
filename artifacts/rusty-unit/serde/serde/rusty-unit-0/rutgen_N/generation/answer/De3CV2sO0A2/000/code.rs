// Answer 0

#[derive(Debug)]
struct MockVisitor {
    value: String,
}

impl<'de> Visitor<'de> for MockVisitor {
    type Value = String;
    type Error = &'static str;

    fn visit_str<E>(self, value: &'de str) -> Result<Self::Value, Self::Error> {
        Ok(value.to_string())
    }
}

struct MockDeserializer {
    value: &'static str,
}

impl MockDeserializer {
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, V::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.value)
    }
}

#[test]
fn test_deserialize_any() {
    let deserializer = MockDeserializer { value: "test string" };
    let visitor = MockVisitor { value: String::new() };
    
    let result = deserializer.deserialize_any(visitor).unwrap();
    assert_eq!(result, "test string");
}

#[test]
fn test_deserialize_empty_string() {
    let deserializer = MockDeserializer { value: "" };
    let visitor = MockVisitor { value: String::new() };

    let result = deserializer.deserialize_any(visitor).unwrap();
    assert_eq!(result, "");
}

