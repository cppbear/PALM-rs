// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use serde_json::json;
    use serde_json::map::{Map, Entry};
    use serde_json::Value;

    // Initialize the map
    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    // Access the entry for the key "serde"
    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            // Insert a new value and capture the old value
            assert_eq!(occupied.insert(json!(13)), json!(12));
            assert_eq!(occupied.get(), &json!(13));
        }
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_insert_multiple_entries() {
    use serde_json::json;
    use serde_json::map::{Map, Entry};
    use serde_json::Value;

    // Initialize the map and populate it
    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(1));
    map.insert("key2".to_owned(), json!(2));

    // Test inserting into the first occupied entry
    match map.entry("key1") {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.insert(json!(10)), json!(1));
            assert_eq!(occupied.get(), &json!(10));
        }
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }

    // Test inserting into the second occupied entry
    match map.entry("key2") {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.insert(json!(20)), json!(2));
            assert_eq!(occupied.get(), &json!(20));
        }
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_insert_non_existent_key() {
    use serde_json::json;
    use serde_json::map::{Map, Entry};
    use serde_json::Value;

    // Initialize the map
    let mut map = Map::new();

    // Check that inserting into a non-existent key should panic
    match map.entry("non_existent_key") {
        Entry::Occupied(_) => panic!("Expected a vacant entry"),
        Entry::Vacant(vacant) => {
            vacant.insert(json!(42)); // Just to check that we can insert and it won't panic
        }
    }
}

