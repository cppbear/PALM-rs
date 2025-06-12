// Answer 0

#[test]
fn test_or_insert_occupied() {
    use serde_json::{Map, Value, json};
    
    // Create a map and insert some initial value
    let mut map = Map::new();
    map.insert("serde".to_string(), json!(42));
    
    // Create an occupied entry
    let entry = map.entry("serde".to_string());
    
    // Call or_insert with a default value
    let value = entry.or_insert(json!(12));
    
    // Ensure the mutable reference points to the correct value
    assert_eq!(*value, json!(42));
    assert_eq!(map["serde"], json!(42));
}

#[test]
fn test_or_insert_vacant() {
    use serde_json::{Map, Value, json};

    // Create a new map
    let mut map = Map::new();
    
    // Create a vacant entry
    let entry = map.entry("serde".to_string());
    
    // Call or_insert with a default value
    let value = entry.or_insert(json!(12));
    
    // Check that the value was inserted correctly
    assert_eq!(*value, json!(12));
    assert_eq!(map["serde"], json!(12));
}

#[test]
#[should_panic]
fn test_or_insert_panic_on_occupied() {
    use serde_json::{Map, Value, json};

    // Create a map with an initial value
    let mut map = Map::new();
    map.insert("serde".to_string(), json!(42));
    
    // Access the entry for serde, which is occupied
    let entry = map.entry("serde".to_string());
    
    // Should panic since we are trying to insert while it is occupied
    let _value = entry.or_insert(json!(12));
}

