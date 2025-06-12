// Answer 0

#[test]
fn test_deserialize_struct_array() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(Value::Array(vec![]))
        }
    }

    let value = Value::Array(vec![Value::String(String::from("test"))]);
    let result = value.deserialize_struct("TestStruct", &["field1"], TestVisitor { value: None });

    assert!(result.is_ok());
}

#[test]
fn test_deserialize_struct_object() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let mut object = serde_json::Map::new();
    object.insert("field1".to_string(), Value::String(String::from("value")));
    let value = Value::Object(object);

    let result = value.deserialize_struct("TestStruct", &["field1"], TestVisitor { value: None });

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_struct_invalid_type() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(Value::Array(vec![]))
        }
    }

    let value = Value::Bool(true);
    let result = value.deserialize_struct("TestStruct", &["field1"], TestVisitor { value: None });

    let _ = result.expect("Expected an error due to invalid type");
}

