// Answer 0

#[test]
fn test_get_mut_existing_key() {
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use std::hash::Hash;

    struct Value {
        data: i32,
    }

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get_mut(key)
        }

        pub fn insert(&mut self, key: String, value: Value) {
            self.map.insert(key, value);
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), Value { data: 10 });

    if let Some(value) = my_map.get_mut("key1") {
        value.data = 20;
    }

    assert_eq!(my_map.map.get("key1").unwrap().data, 20);
}

#[test]
fn test_get_mut_non_existing_key() {
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use std::hash::Hash;

    struct Value {
        data: i32,
    }

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get_mut(key)
        }

        pub fn insert(&mut self, key: String, value: Value) {
            self.map.insert(key, value);
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), Value { data: 10 });

    let value_mut = my_map.get_mut("key2");
    assert!(value_mut.is_none());
}

#[test]
#[should_panic]
fn test_get_mut_panic_on_invalid_borrow() {
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use std::hash::Hash;

    struct Value {
        data: i32,
    }

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get_mut(key)
        }

        pub fn insert(&mut self, key: String, value: Value) {
            self.map.insert(key, value);
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), Value { data: 10 });

    // Attempting to borrow a string slice which won't panic, but we will force a panic for invalid access
    let _value_mut = my_map.get_mut("nonexistent_key").unwrap();
}

