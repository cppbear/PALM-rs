// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(42));

    match map.entry("key1") {
        Entry::Occupied(occupied) => {
            let value = occupied.remove();
            assert_eq!(value, json!(42));
            assert!(map.get("key1").is_none());
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_remove_vacant_entry() {
    use serde_json::Map;
    use serde_json::map::Entry;

    let mut map = Map::new();

    match map.entry("nonexistent_key") {
        Entry::Occupied(_) => panic!("Expected entry to be vacant"),
        Entry::Vacant(vacant) => {
            let value = vacant.remove(); // this should not panic
            assert!(value.is_null()); // Expecting a no-op return
        }
    }
}

#[test]
#[should_panic]
fn test_remove_from_empty_map() {
    use serde_json::Map;
    use serde_json::map::Entry;

    let mut map = Map::new();

    // Attempting to remove from an empty map should trigger a panic when accessing a key
    match map.entry("key") {
        Entry::Occupied(_) => unreachable!(),
        Entry::Vacant(vacant) => {
            let _ = vacant.remove(); // this should not panic, no-op
        }
    }
} 

#[test]
fn test_remove_after_swap_remove() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("key_to_remove".to_owned(), json!(100));
    map.insert("key_to_keep".to_owned(), json!(200));

    // First remove an element
    match map.entry("key_to_remove") {
        Entry::Occupied(occupied) => {
            let value = occupied.remove();
            assert_eq!(value, json!(100));
            assert!(map.get("key_to_remove").is_none());
        }
        _ => panic!("Expected entry to be occupied"),
    }

    // Now check if the other key still exists
    assert_eq!(map.get("key_to_keep"), Some(&json!(200)));
}

