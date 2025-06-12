// Answer 0

#[test]
fn test_get_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;
    
    struct MyMap {
        map: HashMap<String, serde_json::Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get<Q>(&self, key: &Q) -> Option<&serde_json::Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let mut my_map = MyMap::new();
    my_map.map.insert("key1".to_string(), serde_json::json!(1));

    assert_eq!(my_map.get("key1"), Some(&serde_json::json!(1)));
}

#[test]
fn test_get_non_existing_key() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;

    struct MyMap {
        map: HashMap<String, serde_json::Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get<Q>(&self, key: &Q) -> Option<&serde_json::Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let my_map = MyMap::new();

    assert_eq!(my_map.get("non_existing_key"), None);
}

#[test]
fn test_get_key_with_different_borrowed_type() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;

    struct MyMap {
        map: HashMap<String, serde_json::Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get<Q>(&self, key: &Q) -> Option<&serde_json::Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let mut my_map = MyMap::new();
    my_map.map.insert("key2".to_string(), serde_json::json!(2));

    // Test borrowing as a str
    assert_eq!(my_map.get("key2"), Some(&serde_json::json!(2)));
}

#[test]
#[should_panic]
fn test_get_with_panic_condition() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Borrow;

    struct MyMap {
        map: HashMap<String, serde_json::Value>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }
        
        pub fn get<Q>(&self, key: &Q) -> Option<&serde_json::Value>
        where
            String: Borrow<Q>,
            Q: ?Sized + Ord + Eq + Hash,
        {
            self.map.get(key)
        }
    }

    let my_map = MyMap::new();
    
    // Attempt to use a key that doesn't exist and check the panic, depending on the handling
    let _ = my_map.get(&"panic_key"[..]).unwrap(); 
}

