// Answer 0

#[test]
fn test_end_creates_object_with_array() {
    use serde_json::{Value, Map};

    struct TestStruct {
        name: String,
        vec: Vec<Value>,
    }

    impl TestStruct {
        fn new(name: String, vec: Vec<Value>) -> Self {
            TestStruct { name, vec }
        }

        fn end(self) -> Result<Value, serde_json::Error> {
            let mut object = Map::new();
            object.insert(self.name.clone(), Value::Array(self.vec));

            Ok(Value::Object(object))
        }
    }

    let test_data = TestStruct::new(
        "numbers".to_string(),
        vec![Value::Number(serde_json::Number::from(1)), Value::Number(serde_json::Number::from(2))],
    );

    let result = test_data.end().unwrap();
    
    assert!(result.is_object());
    if let Value::Object(obj) = result {
        assert_eq!(obj.len(), 1);
        assert!(obj.contains_key("numbers"));
        if let Some(Value::Array(arr)) = obj.get("numbers") {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected Value::Array for key 'numbers'");
        }
    }
}

