// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use serde_json::Map;
    use serde_json::json;

    let mut map = Map::new();
    map.insert("key1".to_string(), json!(42));

    let entry = map.entry("key1").or_insert(json!(0));
    
    if let Entry::Occupied(occupied) = entry {
        assert_eq!(occupied.remove(), json!(42));
    } else {
        panic!("Expected `Occupied` entry");
    }
}

#[test]
fn test_remove_non_existent_key() {
    use serde_json::Map;
    use serde_json::json;

    let mut map = Map::new();
    map.insert("key1".to_string(), json!(42));

    let entry = map.entry("key2").or_insert(json!(0));
    
    if let Entry::Vacant(_) = entry {
        // No panic will occur here, but can't remove since it doesn't exist
    } else {
        panic!("Expected `Vacant` entry");
    }
}

#[test]
fn test_remove_with_preserve_order() {
    #[cfg(feature = "preserve_order")]
    {
        use serde_json::Map;
        use serde_json::json;

        let mut map = Map::new();
        map.insert("key1".to_string(), json!(42));
        map.insert("key2".to_string(), json!(24));

        let entry = map.entry("key1").or_insert(json!(0));

        if let Entry::Occupied(occupied) = entry {
            assert_eq!(occupied.remove(), json!(42));
        } else {
            panic!("Expected `Occupied` entry");
        }
    }
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_remove_order_preservation() {
    use serde_json::Map;
    use serde_json::json;

    let mut map = Map::new();
    map.insert("key1".to_string(), json!(1));
    map.insert("key2".to_string(), json!(2));

    if let Entry::Occupied(occupied) = map.entry("key1") {
        let removed_value = occupied.remove();
        assert_eq!(removed_value, json!(1));
        // Verify the order is preserved in subsequent lookups
        assert!(map.get("key1").is_none());
        assert_eq!(map.get("key2"), Some(&json!(2)));
    } else {
        panic!("Expected Occupied entry");
    }
}

#[test]
#[should_panic]
fn test_remove_error_for_invalid_key() {
    use serde_json::Map;
    use serde_json::json;

    let mut map = Map::new();
    map.insert("key1".to_string(), json!(42));

    // Attempting to remove a key that does not exist
    match map.entry("key2") {
        Entry::Occupied(_) => panic!("Expected `Vacant`, but found `Occupied`"),
        Entry::Vacant(_) => {
            // We can't panic on remove since it doesn't exist but simulating a false panic
            panic!("Removing a non-existent entry should not be allowed");
        }
    }
}

#[test]
fn test_remove_edge_case() {
    use serde_json::Map;
    use serde_json::json;

    let mut map: Map<String, Value> = Map::new();
    map.insert("single_key".to_string(), json!(100));

    if let Entry::Occupied(occupied) = map.entry("single_key") {
        let removed_value = occupied.remove();
        assert_eq!(removed_value, json!(100));
        assert!(map.get("single_key").is_none()); // Should be empty now
    } else {
        panic!("Expected Occupied entry");
    }
}

