// Answer 0

#[test]
fn test_deserialize_struct_with_object() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct DummyVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a JSON object")
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let object_value = Value::Object(serde_json::Map::new());
    let result = object_value.deserialize_struct("test", &["field1", "field2"], DummyVisitor { value: None });
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_struct_with_array_fails() {
    use serde_json::Value;

    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a JSON object")
        }
    }

    let array_value = Value::Array(vec![]);
    let result = array_value.deserialize_struct("test", &["field1", "field2"], DummyVisitor);
    assert!(result.is_err());
}

