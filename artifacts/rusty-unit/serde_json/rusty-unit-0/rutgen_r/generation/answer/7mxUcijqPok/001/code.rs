// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use serde_json::{json, Map, Value};
    
    // Create a new `Map` and insert initial values
    let mut map = Map::new();
    map.insert("serde".to_string(), json!("cpp"));
    
    // Create an `Entry` by calling .entry() method on the map
    let entry = map.entry("serde".to_string());

    // Test and_modify to ensure it modifies the value for occupied entry
    let modified_entry = entry.and_modify(|e| *e = json!("rust"));

    // Check if we get an Occupied entry back
    match modified_entry {
        Entry::Occupied(_) => assert_eq!(map["serde"], "rust"),
        Entry::Vacant(_) => panic!("Expected an Occupied entry"),
    }
}

#[test]
fn test_and_modify_with_panic() {
    use serde_json::{json, Map};

    // Create a new `Map` and insert value
    let mut map = Map::new();
    map.insert("serde".to_string(), json!("cpp"));

    // Create an `Entry` for a non-existing key
    let entry = map.entry("not_existing".to_string());

    // Call and_modify on vacant entry, expecting no modification
    entry.and_modify(|_| panic!("This should not be called"));

    // Verify that value has not changed
    assert_eq!(map["serde"], "cpp");
}

