// Answer 0

#[test]
fn test_end_creates_object() {
    use serde_json::{Value, Map};

    struct TestStruct {
        name: String,
        map: Map<String, Value>,
    }

    impl TestStruct {
        fn new(name: String, map: Map<String, Value>) -> Self {
            Self { name, map }
        }

        fn end(self) -> Result<Value, serde_json::Error> {
            let mut object = Map::new();
            object.insert(self.name, Value::Object(self.map));
            Ok(Value::Object(object))
        }
    }

    let mut map = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));
    
    let test_struct = TestStruct::new("test".to_string(), map);
    
    let result = test_struct.end().unwrap();
    
    if let Value::Object(obj) = result {
        assert!(obj.contains_key("test"));
        if let Value::Object(inner_map) = obj.get("test").unwrap() {
            assert!(inner_map.contains_key("key"));
            assert_eq!(inner_map.get("key").unwrap(), &Value::String("value".to_string()));
        } else {
            panic!("Expected inner map to be an Object");
        }
    } else {
        panic!("Expected result to be an Object");
    }
}

#[test]
#[should_panic]
fn test_end_panics_on_empty_name() {
    use serde_json::{Value, Map};

    struct TestStruct {
        name: String,
        map: Map<String, Value>,
    }

    impl TestStruct {
        fn new(name: String, map: Map<String, Value>) -> Self {
            Self { name, map }
        }

        fn end(self) -> Result<Value, serde_json::Error> {
            if self.name.is_empty() {
                panic!("Name cannot be empty");
            }
            let mut object = Map::new();
            object.insert(self.name, Value::Object(self.map));
            Ok(Value::Object(object))
        }
    }

    let mut map = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));

    let test_struct = TestStruct::new("".to_string(), map);
    test_struct.end().unwrap();
}

