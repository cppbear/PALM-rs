// Answer 0

#[test]
fn test_serialize_newtype_variant_with_integer() {
    use serde::Serialize;
    use serde_json::{json, Value, Map};

    fn serialize_newtype_variant<T>(
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, serde_json::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), serde_json::to_value(value)?);
        Ok(Value::Object(values))
    }

    let result = serialize_newtype_variant("MyType", 0, "MyVariant", &42);
    let expected = json!({"MyVariant": 42});
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_serialize_newtype_variant_with_string() {
    use serde::Serialize;
    use serde_json::{json, Value, Map};

    fn serialize_newtype_variant<T>(
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, serde_json::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), serde_json::to_value(value)?);
        Ok(Value::Object(values))
    }

    let result = serialize_newtype_variant("MyType", 0, "MyVariant", &"Hello");
    let expected = json!({"MyVariant": "Hello"});
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_serialize_newtype_variant_with_empty_string() {
    use serde::Serialize;
    use serde_json::{json, Value, Map};

    fn serialize_newtype_variant<T>(
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, serde_json::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), serde_json::to_value(value)?);
        Ok(Value::Object(values))
    }

    let result = serialize_newtype_variant("MyType", 0, "MyVariant", &"");
    let expected = json!({"MyVariant": ""});
    assert_eq!(result.unwrap(), expected);
}

