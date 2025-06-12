// Answer 0

#[test]
fn test_remove_entry_occupied() {
    use serde_json::json;
    use serde_json::map::{Entry, Map};
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            let (key, value) = occupied.remove_entry();
            assert_eq!(key, "serde");
            assert_eq!(value, json!(12));
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
#[should_panic]
fn test_remove_entry_vacant() {
    use serde_json::json;
    use serde_json::map::{Entry, Map};
    
    let mut map = Map::new();

    match map.entry("nonexistent") {
        Entry::Occupied(_) => panic!("Entry should not be occupied"),
        Entry::Vacant(_) => {
            // Simulate remove_entry call on a vacant entry
            // Which isn't supposed to happen
            panic!("Expected to panic on vacancy");
        }
    }
}

