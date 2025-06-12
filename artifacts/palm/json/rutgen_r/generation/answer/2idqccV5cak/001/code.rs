// Answer 0

#[test]
fn test_end_function_with_valid_data() {
    use serde_json::{Map, Value};

    struct TestStruct {
        name: String,
        vec: Vec<Value>,
    }

    impl TestStruct {
        fn new(name: &str, vec: Vec<Value>) -> Self {
            TestStruct {
                name: name.to_string(),
                vec,
            }
        }

        fn end(self) -> Result<Value, serde_json::Error> {
            let mut object = Map::new();
            object.insert(self.name.clone(), Value::Array(self.vec));
            Ok(Value::Object(object))
        }
    }

    let valid_vec = vec![Value::from(1), Value::from(2)];
    let test_instance = TestStruct::new("test_key", valid_vec);
    let result = test_instance.end();

    assert!(result.is_ok());
    if let Ok(value) = result {
        if let Value::Object(ref object) = value {
            assert!(object.contains_key("test_key"));
            if let Some(Value::Array(ref array)) = object.get("test_key") {
                assert_eq!(array.len(), 2);
                assert_eq!(array[0], Value::from(1));
                assert_eq!(array[1], Value::from(2));
            } else {
                panic!("Expected Value::Array for 'test_key'");
            }
        } else {
            panic!("Expected Value::Object");
        }
    }
}

