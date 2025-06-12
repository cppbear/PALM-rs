// Answer 0

#[test]
fn test_remove_entry_key_present() {
    use std::collections::HashMap;
    use std::hash::Hash;

    let mut map: HashMap<String, serde_json::Value> = HashMap::new();
    map.insert("key1".to_string(), serde_json::json!(1));
    map.insert("key2".to_string(), serde_json::json!(2));

    let result = map.remove_entry("key1");

    assert_eq!(result, Some(("key1".to_string(), serde_json::json!(1))));
    assert!(!map.contains_key("key1"));
}

#[test]
fn test_remove_entry_key_absent() {
    use std::collections::HashMap;
    use std::hash::Hash;

    let mut map: HashMap<String, serde_json::Value> = HashMap::new();
    map.insert("key1".to_string(), serde_json::json!(1));
    
    let result = map.remove_entry("key2");
    
    assert_eq!(result, None);
    assert!(map.contains_key("key1"));
}

#[test]
fn test_remove_entry_empty_map() {
    use std::collections::HashMap;
    use std::hash::Hash;

    let mut map: HashMap<String, serde_json::Value> = HashMap::new();
    
    let result = map.remove_entry("key1");
    
    assert_eq!(result, None);
}

#[should_panic]
fn test_remove_entry_with_panic() {
    use std::collections::HashMap;
    use std::hash::Hash;

    let mut map: HashMap<String, serde_json::Value> = HashMap::new();
    map.insert("key1".to_string(), serde_json::json!(1));
    
    // This panics as "not_found_key" is not in the map.
    let _result = map.remove_entry("not_found_key");
}

#[test]
fn test_remove_entry_with_different_ordering() {
    use std::collections::HashMap;
    use std::hash::Hash;

    let mut map: HashMap<String, serde_json::Value> = HashMap::new();
    map.insert("apple".to_string(), serde_json::json!(1));
    map.insert("banana".to_string(), serde_json::json!(2));
    
    // Remove in different case
    let result = map.remove_entry("Apple");
    
    assert_eq!(result, None);
    assert!(map.contains_key("apple"));
}

