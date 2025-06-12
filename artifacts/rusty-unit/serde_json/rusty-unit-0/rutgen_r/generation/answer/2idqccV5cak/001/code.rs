// Answer 0

#[test]
fn test_end_function_with_valid_input() {
    use serde_json::{Value, Map};

    struct TestStruct {
        name: String,
        vec: Vec<Value>,
    }

    impl TestStruct {
        fn end(self) -> Result<Value, &'static str> {
            let mut object = Map::new();
            object.insert(self.name, Value::Array(self.vec));
            Ok(Value::Object(object))
        }
    }

    let test_struct = TestStruct {
        name: String::from("key"),
        vec: vec![Value::Number(1.into()), Value::Number(2.into())],
    };

    let result = test_struct.end().unwrap();
    let expected = {
        let mut object = Map::new();
        object.insert("key".to_string(), Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]));
        Value::Object(object)
    };

    assert_eq!(result, expected);
}

#[test]
fn test_end_function_with_empty_vector() {
    use serde_json::{Value, Map};

    struct TestStruct {
        name: String,
        vec: Vec<Value>,
    }

    impl TestStruct {
        fn end(self) -> Result<Value, &'static str> {
            let mut object = Map::new();
            object.insert(self.name, Value::Array(self.vec));
            Ok(Value::Object(object))
        }
    }

    let test_struct = TestStruct {
        name: String::from("empty_key"),
        vec: vec![],
    };

    let result = test_struct.end().unwrap();
    let expected = {
        let mut object = Map::new();
        object.insert("empty_key".to_string(), Value::Array(vec![]));
        Value::Object(object)
    };

    assert_eq!(result, expected);
}

