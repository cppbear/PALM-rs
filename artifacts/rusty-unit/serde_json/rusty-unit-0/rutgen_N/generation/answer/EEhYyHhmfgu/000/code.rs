// Answer 0

#[derive(Debug)]
struct TestVisitor {
    value: Option<bool>,
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = bool;

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean value")
    }
}

struct TestDeserializer {
    key: &'static str,
}

impl TestDeserializer {
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.key == "true" {
            visitor.visit_bool(true)
        } else if self.key == "false" {
            visitor.visit_bool(false)
        } else {
            Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(self.key),
                &visitor,
            ))
        }
    }
}

#[test]
fn test_deserialize_bool_true() {
    let deserializer = TestDeserializer { key: "true" };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_bool(visitor).unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_deserialize_bool_false() {
    let deserializer = TestDeserializer { key: "false" };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_bool(visitor).unwrap();
    assert_eq!(result, false);
}

#[test]
fn test_deserialize_bool_invalid() {
    let deserializer = TestDeserializer { key: "invalid" };
    let visitor = TestVisitor { value: None };
    let result: Result<bool, _> = deserializer.deserialize_bool(visitor);
    assert!(result.is_err());
}

