// Answer 0

#[test]
fn test_entry_occupied_key() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            assert_eq!(occupied.key(), &"serde".to_owned());
        }
        Entry::Vacant(_) => panic!("Expected occupied entry, found vacant."),
    }
}

#[test]
fn test_entry_vacant() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();

    match map.entry("non_existent_key") {
        Entry::Occupied(_) => panic!("Expected vacant entry, found occupied."),
        Entry::Vacant(vacant) => {
            assert_eq!(vacant.key(), &"non_existent_key".to_owned());
        }
    }
}

