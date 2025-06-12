// Answer 0

#[test]
fn test_and_modify_for_occupied_entry() {
    use serde_json::{json, Map, Value, Entry};

    let mut map: Map<String, Value> = Map::new();
    map.insert("serde".to_string(), json!("cpp"));
    
    // Test modifying an existing entry
    map.entry("serde".to_string())
        .and_modify(|e| *e = json!("rust"))
        .or_insert(json!("cpp"));
    
    assert_eq!(map["serde"], json!("rust"));
}

#[test]
fn test_and_modify_for_vacant_entry() {
    use serde_json::{json, Map, Value, Entry};

    let mut map: Map<String, Value> = Map::new();
    
    // Test inserting into a vacant entry
    map.entry("serde".to_string())
        .and_modify(|e| *e = json!("rust"))
        .or_insert(json!("cpp"));
    
    assert_eq!(map["serde"], json!("cpp"));
}

#[test]
fn test_and_modify_entry_twice() {
    use serde_json::{json, Map, Value, Entry};

    let mut map: Map<String, Value> = Map::new();
    map.insert("serde".to_string(), json!("cpp"));
    
    // First modification
    map.entry("serde".to_string())
        .and_modify(|e| *e = json!("rust"))
        .or_insert(json!("cpp"));
    
    assert_eq!(map["serde"], json!("rust"));
    
    // Second modification
    map.entry("serde".to_string())
        .and_modify(|e| *e = json!("rust again"))
        .or_insert(json!("cpp"));
    
    assert_eq!(map["serde"], json!("rust again"));
}

