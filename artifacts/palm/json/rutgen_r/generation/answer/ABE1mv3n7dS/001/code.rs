// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(100));

    match map.entry("key1") {
        Entry::Occupied(occupied) => {
            let value = occupied.remove();
            assert_eq!(value, json!(100));
            assert!(!map.contains_key("key1"));
        }
        Entry::Vacant(_) => panic!("Expected the entry to be occupied"),
    }
}

#[test]
#[should_panic]
fn test_remove_vacant_entry() {
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("non_existing_key") {
        Entry::Occupied(occupied) => {
            let _value = occupied.remove(); // This should not happen
        }
        Entry::Vacant(_) => {
            // Properly handles the empty case - just to reach here
        }
    }
}

