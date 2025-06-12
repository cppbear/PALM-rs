// Answer 0

#[derive(Debug)]
struct MyMap {
    map: std::collections::HashMap<String, i32>,
}

impl MyMap {
    pub fn keys(&self) -> std::collections::hash_map::Keys<String, i32> {
        self.map.keys()
    }
}

#[test]
fn test_empty_map_keys() {
    let my_map = MyMap {
        map: std::collections::HashMap::new(),
    };
    let keys: Vec<&String> = my_map.keys().collect();
    assert_eq!(keys.len(), 0);
}

#[test]
fn test_single_entry_map_keys() {
    let mut my_map = MyMap {
        map: std::collections::HashMap::new(),
    };
    my_map.map.insert("key1".to_string(), 1);
    let keys: Vec<&String> = my_map.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], &"key1".to_string());
}

#[test]
fn test_multiple_entries_map_keys() {
    let mut my_map = MyMap {
        map: std::collections::HashMap::new(),
    };
    my_map.map.insert("key1".to_string(), 1);
    my_map.map.insert("key2".to_string(), 2);
    my_map.map.insert("key3".to_string(), 3);
    
    let keys: Vec<&String> = my_map.keys().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&&"key1".to_string()));
    assert!(keys.contains(&&"key2".to_string()));
    assert!(keys.contains(&&"key3".to_string()));
}

#[test]
fn test_keys_from_large_map() {
    let mut my_map = MyMap {
        map: std::collections::HashMap::new(),
    };
    for i in 0..1000 {
        my_map.map.insert(format!("key{}", i), i);
    }
    
    let keys: Vec<&String> = my_map.keys().collect();
    assert_eq!(keys.len(), 1000);
    for i in 0..1000 {
        assert!(keys.contains(&&format!("key{}", i)));
    }
}

#[test]
fn test_keys_with_duplicates_in_insertions() {
    let mut my_map = MyMap {
        map: std::collections::HashMap::new(),
    };
    my_map.map.insert("key1".to_string(), 1);
    my_map.map.insert("key1".to_string(), 2); // Duplicate key; should overwrite previous value.
    
    let keys: Vec<&String> = my_map.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], &"key1".to_string());
}

