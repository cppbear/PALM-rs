// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "serde");
            assert_eq!(value, json!(12));
        }
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

#[test]
#[should_panic(expected = "Expected an occupied entry")]
fn test_remove_entry_non_existing_key() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));
    
    match map.entry("non_existing") {
        Entry::Occupied(_) => panic!("Expected a vacant entry"),
        Entry::Vacant(_) => {
            // No entry to remove.
        }
    }
}

#[test]
fn test_remove_entry_multiple_keys() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("first".to_owned(), json!(1));
    map.insert("second".to_owned(), json!(2));

    match map.entry("first") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "first");
            assert_eq!(value, json!(1));
        }
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }

    // Validate remaining key
    assert_eq!(map.get("second"), Some(&json!(2)));
}

