// Answer 0

#[test]
fn test_end_map() {
    use serde::ser::{Serialize, Serializer};
    use serde_json::Value;
    
    struct TestMap {
        data: Vec<(String, Value)>,
    }
    
    impl Serialize for TestMap {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.data.len()))?;
            for (key, value) in &self.data {
                map.serialize_entry(key, value)?;
            }
            map.end()
        }
    }

    let map = TestMap {
        data: vec![
            ("key1".to_string(), Value::String("value1".to_string())),
            ("key2".to_string(), Value::Number(serde_json::Number::from(2))),
        ],
    };

    let result = serde_json::to_value(map).unwrap();
    assert_eq!(result["key1"], "value1");
    assert_eq!(result["key2"], 2);
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_end_number() {
    use serde_json::Value;

    struct TestNumber {
        out_value: Option<Value>,
    }

    impl TestNumber {
        fn new(value: Value) -> Self {
            TestNumber { out_value: Some(value) }
        }

        fn end(self) -> Result<Value, &'static str> {
            match self {
                _ => Ok(self.out_value.expect("number value was not emitted")),
            }
        }
    }

    let number = TestNumber::new(Value::Number(serde_json::Number::from(3)));
    let result = number.end().unwrap();
    assert_eq!(result, Value::Number(serde_json::Number::from(3)));
}

#[test]
#[cfg(feature = "raw_value")]
fn test_end_raw_value() {
    use serde_json::Value;

    struct TestRawValue {
        out_value: Option<Value>,
    }

    impl TestRawValue {
        fn new(value: Value) -> Self {
            TestRawValue { out_value: Some(value) }
        }

        fn end(self) -> Result<Value, &'static str> {
            match self {
                _ => Ok(self.out_value.expect("raw value was not emitted")),
            }
        }
    }

    let raw_value = TestRawValue::new(Value::String("raw".to_string()));
    let result = raw_value.end().unwrap();
    assert_eq!(result, Value::String("raw".to_string()));
}

