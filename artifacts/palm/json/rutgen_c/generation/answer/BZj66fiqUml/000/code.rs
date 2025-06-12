// Answer 0

#[test]
fn test_occupied_entry_get() {
    use serde_json::json;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("test_key".to_owned(), json!(42));

    match map.entry("test_key") {
        serde_json::map::Entry::Occupied(occupied) => {
            assert_eq!(occupied.get(), &json!(42));
        }
        serde_json::map::Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_occupied_entry_get_nonexistent() {
    use serde_json::json;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("nonexistent_key") {
        serde_json::map::Entry::Occupied(_) => panic!("Expected vacant entry"),
        serde_json::map::Entry::Vacant(vacant) => {
            vacant.insert(json!(100));
        }
    }

    assert_eq!(map.get("nonexistent_key"), Some(&json!(100)));
}

