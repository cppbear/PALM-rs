// Answer 0

#[derive(Debug)]
struct DummyVisitor {
    value: Option<&'static str>,
}

impl<'de> serde::de::Visitor<'de> for DummyVisitor {
    type Value = &'de str;

    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, serde::de::Error> {
        Ok(value)
    }
}

#[derive(Debug)]
struct DummyDeserializer {
    value: &'static str,
}

impl<'de> serde::de::Deserializer<'de> for DummyDeserializer {
    type Error = serde::de::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.value)
    }
}

#[test]
fn test_deserialize_any() {
    let deserializer = DummyDeserializer { value: "test" };
    let visitor = DummyVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_any_empty() {
    let deserializer = DummyDeserializer { value: "" };
    let visitor = DummyVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), "");
}

