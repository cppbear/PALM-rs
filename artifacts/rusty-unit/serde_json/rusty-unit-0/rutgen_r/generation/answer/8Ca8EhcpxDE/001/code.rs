// Answer 0

#[test]
fn test_end_success_case() {
    use serde_json::{Map, Value};

    struct TestStruct {
        name: String,
        map: Map<String, Value>,
    }

    impl TestStruct {
        fn new(name: &str, map: Map<String, Value>) -> Self {
            TestStruct {
                name: name.to_string(),
                map,
            }
        }

        fn end(self) -> Result<Value, serde_json::Error> {
            let mut object = Map::new();
            object.insert(self.name, Value::Object(self.map));
            Ok(Value::Object(object))
        }
    }

    let mut inner_map = Map::new();
    inner_map.insert("key1".to_string(), Value::from("value1"));
    inner_map.insert("key2".to_string(), Value::from(42));

    let test_struct = TestStruct::new("test_name", inner_map);
    let result = test_struct.end();

    assert!(result.is_ok());
    let value = result.unwrap();
    if let Value::Object(obj) = value {
        assert!(obj.contains_key("test_name"));
        if let Value::Object(inner_obj) = obj.get("test_name").unwrap() {
            assert_eq!(inner_obj.get("key1").unwrap(), &Value::from("value1"));
            assert_eq!(inner_obj.get("key2").unwrap(), &Value::from(42));
        } else {
            panic!("Expected Value::Object for test_name");
        }
    } else {
        panic!("Expected Value::Object as the return type");
    }
}

