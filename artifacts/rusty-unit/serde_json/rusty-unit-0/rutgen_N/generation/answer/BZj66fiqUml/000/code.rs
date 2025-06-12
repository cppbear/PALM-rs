// Answer 0

#[test]
fn test_entry_occupied_get() {
    use serde_json::json;
    use serde_json::Map;
    use serde_json::map::Entry;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            assert_eq!(occupied.get(), &json!(12));
        }
        Entry::Vacant(_) => panic!("Expected to find an occupied entry."),
    }
}

#[test]
fn test_entry_vacant_get() {
    use serde_json::json;
    use serde_json::Map;
    use serde_json::map::Entry;

    let mut map = Map::new();

    match map.entry("serde") {
        Entry::Occupied(_) => panic!("Expected to find a vacant entry."),
        Entry::Vacant(_) => {
            // Vacant entry cannot get a value, this is just to verify functionality.
        }
    }
}

