// Answer 0

#[test]
fn test_remove_entry() {
    use serde_json::json;
    use serde_json::Map;

    // Create a new map and insert a value
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_owned(), json!(100));

    // Retrieve the occupied entry
    match map.entry("key1") {
        serde_json::map::Entry::Occupied(occupied) => {
            // Remove entry and capture the result
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "key1");
            assert_eq!(value, json!(100));
            // Confirm the map is now empty
            assert!(map.is_empty());
        }
        serde_json::map::Entry::Vacant(_) => panic!("Entry should be occupied"),
    }
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_remove_entry_preserve_order() {
    use serde_json::json;
    use serde_json::Map;

    // Create a new map and insert some values
    let mut map: Map<String, Value> = Map::new();
    map.insert("first".to_owned(), json!(1));
    map.insert("second".to_owned(), json!(2));

    // Retrieve the occupied entry for "second"
    match map.entry("second") {
        serde_json::map::Entry::Occupied(occupied) => {
            // Remove entry and capture the result
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "second");
            assert_eq!(value, json!(2));
            // Confirm the map still has the other entry
            assert_eq!(map.len(), 1);
            assert!(map.contains_key("first"));
        }
        serde_json::map::Entry::Vacant(_) => panic!("Entry should be occupied"),
    }
}

