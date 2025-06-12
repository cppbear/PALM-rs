// Answer 0

#[test]
fn test_end_with_map() {
    struct TestSerializeMap;

    impl serde::ser::SerializeMap for TestSerializeMap {
        fn end(self) -> Result<Value> {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let map = TestSerializeMap;
    let result = map.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Object(serde_json::Map::new()));
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_end_with_number() {
    struct TestSerializeMap {
        out_value: Option<Value>,
    }

    impl serde::ser::SerializeMap for TestSerializeMap {
        fn end(self) -> Result<Value> {
            Ok(self.out_value.expect("number value was not emitted"))
        }
    }

    let map = TestSerializeMap { out_value: Some(Value::Number(serde_json::Number::from(42))) };
    let result = map.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(serde_json::Number::from(42)));
}

#[cfg(feature = "raw_value")]
#[test]
fn test_end_with_raw_value() {
    struct TestSerializeMap {
        out_value: Option<Value>,
    }

    impl serde::ser::SerializeMap for TestSerializeMap {
        fn end(self) -> Result<Value> {
            Ok(self.out_value.expect("raw value was not emitted"))
        }
    }

    let map = TestSerializeMap { out_value: Some(Value::String("raw_value".to_string())) };
    let result = map.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::String("raw_value".to_string()));
}

