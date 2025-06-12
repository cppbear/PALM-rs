// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map: Map<String, Value> = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            let removed_value = occupied.remove();
            assert_eq!(removed_value, json!(12));
            assert!(map.get("serde").is_none());
        }
        Entry::Vacant(_) => panic!("Entry should be occupied"),
    }
}

#[test]
fn test_remove_vacant_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map: Map<String, Value> = Map::new();

    match map.entry("not_found") {
        Entry::Occupied(_) => panic!("Entry should be vacant"),
        Entry::Vacant(vacant) => {
            // Can't remove a vacant entry, so we simply assert it exists
            assert!(vacant.key() == "not_found");
        }
    }
}

