// Answer 0

#[test]
fn test_get_on_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            let value: &Value = occupied.get();
            assert_eq!(value, &json!(12));
        }
        Entry::Vacant(_) => panic!("Expected occupied entry, got vacant."),
    }
}

#[test]
fn test_get_on_vacant_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();

    match map.entry("not_exist") {
        Entry::Occupied(_) => panic!("Expected vacant entry, got occupied."),
        Entry::Vacant(_) => {
            // Vacant entry should not panic and does not have a value to get
            // This case is used just to validate that the method works for a vacant entry.
        }
    }
}

