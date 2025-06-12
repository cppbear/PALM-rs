// Answer 0

#[test]
fn test_remove_entry_occupied() {
    use serde_json::{json, Value};
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();
    map.insert("key1".to_owned(), json!(100));

    match map.entry("key1") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "key1");
            assert_eq!(value, json!(100));
        }
        Entry::Vacant(_) => panic!("Entry should be occupied"),
    }
}

#[test]
fn test_remove_entry_vacant() {
    use serde_json::{json, Value};
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();

    match map.entry("key2") {
        Entry::Occupied(_) => panic!("Entry should be vacant"),
        Entry::Vacant(vacant) => {
            assert!(vacant.insert(json!(200)).is_none());
        }
    }

    // Now we can remove the inserted entry
    match map.entry("key2") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "key2");
            assert_eq!(value, json!(200));
        }
        Entry::Vacant(_) => panic!("Entry should be occupied after insertion"),
    }
}

