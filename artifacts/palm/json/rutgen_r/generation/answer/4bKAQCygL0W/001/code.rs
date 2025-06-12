// Answer 0

#[test]
fn test_insert_new_key() {
    use std::collections::HashMap;
    use serde_json::Value;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let result = my_map.insert("key1".to_string(), Value::from(1));
    assert_eq!(result, None);
}

#[test]
fn test_insert_existing_key() {
    use std::collections::HashMap;
    use serde_json::Value;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), Value::from(1));
    let old_value = my_map.insert("key1".to_string(), Value::from(2));
    assert_eq!(old_value, Some(Value::from(1)));
}

#[test]
fn test_insert_with_empty_key() {
    use std::collections::HashMap;
    use serde_json::Value;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let result = my_map.insert("".to_string(), Value::from(1));
    assert_eq!(result, None);
}

#[test]
fn test_insert_value_none() {
    use std::collections::HashMap;
    use serde_json::Value;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
            self.map.insert(k, v)
        }
    }

    let mut my_map = MyMap::new();
    let key = "key_none".to_string();
    let result = my_map.insert(key.clone(), Value::Null);
    assert_eq!(result, None);
    let old_value = my_map.insert(key.clone(), Value::Null);
    assert_eq!(old_value, Some(Value::Null));
}

