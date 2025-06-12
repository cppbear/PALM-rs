// Answer 0

#[test]
fn test_deserialize_any_with_object() {
    use serde_json::Value;
    use serde::de::{self, Visitor};
    
    struct MockVisitor {
        pub value: Option<Value>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Value;
        
        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(Value::Null)
        }

        fn visit_bool(self, v: bool) -> Result<Self::Value, de::Error> {
            Ok(Value::Bool(v))
        }

        fn visit_string(self, v: String) -> Result<Self::Value, de::Error> {
            Ok(Value::String(v))
        }

        fn visit_array(self, _v: &[Value]) -> Result<Self::Value, de::Error> {
            Ok(Value::Array(vec![]))
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let object_val = Value::Object(serde_json::Map::new());
    let visitor = MockVisitor { value: None };
    
    let result = object_val.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Object(serde_json::Map::new()));
}

