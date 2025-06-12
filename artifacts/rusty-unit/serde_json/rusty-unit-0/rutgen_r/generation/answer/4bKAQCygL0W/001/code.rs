// Answer 0

#[test]
fn test_insert_new_key() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let key = "key1".to_string();
    let value = Value::from("value1");
    
    let result = my_map.insert(key.clone(), value.clone());
    
    assert_eq!(result, None);
    assert_eq!(my_map.map.get(&key), Some(&value));
}

#[test]
fn test_insert_existing_key() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let key = "key2".to_string();
    let initial_value = Value::from("initial_value");
    
    my_map.insert(key.clone(), initial_value.clone());
    let new_value = Value::from("new_value");
    
    let result = my_map.insert(key.clone(), new_value.clone());
    
    assert_eq!(result, Some(initial_value));
    assert_eq!(my_map.map.get(&key), Some(&new_value));
}

#[test]
fn test_insert_empty_key() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let key = "".to_string();
    let value = Value::from("empty_key_value");
    
    let result = my_map.insert(key.clone(), value.clone());
    
    assert_eq!(result, None);
    assert_eq!(my_map.map.get(&key), Some(&value));
}

#[test]
fn test_insert_special_characters() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let key = "#$%^&*()".to_string();
    let value = Value::from("specialCharactersValue");
    
    let result = my_map.insert(key.clone(), value.clone());
    
    assert_eq!(result, None);
    assert_eq!(my_map.map.get(&key), Some(&value));
}

