// Answer 0

#[test]
fn test_keys_empty_map() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, i32> {
            self.map.keys()
        }
    }

    let my_map = MyMap { map: HashMap::new() };
    let mut keys_iter = my_map.keys();

    assert!(keys_iter.next().is_none()); // No keys should be present
}

#[test]
fn test_keys_single_element_map() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, i32> {
            self.map.keys()
        }
    }

    let mut my_map = MyMap { map: HashMap::new() };
    my_map.map.insert("key1".to_string(), 1);

    let mut keys_iter = my_map.keys();
    
    assert_eq!(keys_iter.next(), Some(&"key1".to_string())); // Should return the existing key
    assert!(keys_iter.next().is_none()); // There are no more keys
}

#[test]
fn test_keys_multiple_elements_map() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, i32> {
            self.map.keys()
        }
    }

    let mut my_map = MyMap { map: HashMap::new() };
    my_map.map.insert("key1".to_string(), 1);
    my_map.map.insert("key2".to_string(), 2);
    my_map.map.insert("key3".to_string(), 3);

    let keys: Vec<_> = my_map.keys().cloned().collect();
    
    assert_eq!(keys.len(), 3); // Should be 3 keys
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
    assert!(keys.contains(&"key3".to_string()));
}

