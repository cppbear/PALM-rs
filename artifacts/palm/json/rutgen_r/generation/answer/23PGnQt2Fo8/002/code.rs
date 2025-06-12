// Answer 0

#[test]
fn test_deserialize_enum_with_valid_enum() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
 
    struct MyVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = Value;

        fn visit_enum<E>(self, _deserializer: E) -> Result<Self::Value, E::Error>
        where
            E: serde::de::EnumAccess<'de>,
        {
            Ok(self.value.unwrap()) // returning the value wrapped in MyVisitor
        }
    }

    let json_map: Value = serde_json::json!({
        "variant": "example_variant",
        "value": "example_value"
    });

    let deser_result: Result<Value, _> = deserialize_enum(
        json_map.as_object().unwrap().iter().map(|(k, v)| (k, v.clone())),
        "example_enum",
        &["example_variant"],
        MyVisitor { value: Some(Value::String("example_value".to_string())) }
    );

    assert!(deser_result.is_ok());
    assert_eq!(deser_result.unwrap(), Value::String("example_value".to_string()));
}

#[test]
#[should_panic(expected = "invalid value")]
fn test_deserialize_enum_with_empty_map() {
    use serde_json::Value;

    let json_map: Value = serde_json::json!({}); // Empty map
    let _: Result<Value, _> = deserialize_enum(
        json_map.as_object().unwrap().iter().map(|(k, v)| (k, v.clone())),
        "example_enum",
        &["example_variant"],
        MyVisitor { value: None }
    );
}

#[test]
#[should_panic(expected = "invalid value")]
fn test_deserialize_enum_with_extra_keys() {
    use serde_json::Value;

    let json_map: Value = serde_json::json!({
        "variant": "example_variant",
        "value": "example_value",
        "extra_key": "extra_value"
    });

    let _: Result<Value, _> = deserialize_enum(
        json_map.as_object().unwrap().iter().map(|(k, v)| (k, v.clone())),
        "example_enum",
        &["example_variant"],
        MyVisitor { value: Some(Value::String("example_value".to_string())) }
    );
}

