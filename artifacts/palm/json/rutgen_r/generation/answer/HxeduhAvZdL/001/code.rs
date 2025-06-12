// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.insert(json!(13)), json!(12));
            assert_eq!(occupied.get(), &json!(13));
        }
        Entry::Vacant(_) => panic!("Expected occupied entry, but found vacant."),
    }
}

#[test]
fn test_insert_multiple_occurrences() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();
    map.insert("key1".to_owned(), json!(1));

    match map.entry("key1") {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.insert(json!(2)), json!(1));
            assert_eq!(occupied.insert(json!(3)), json!(2));
            assert_eq!(occupied.get(), &json!(3));
        }
        Entry::Vacant(_) => panic!("Expected occupied entry, but found vacant."),
    }
}

#[test]
fn test_insert_non_existing_key() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();

    match map.entry("non_existent_key") {
        Entry::Occupied(_) => panic!("Expected vacant entry, but found occupied."),
        Entry::Vacant(vacant) => {
            vacant.insert(json!(42));
            assert_eq!(map.get("non_existent_key"), Some(&json!(42)));
        }
    }
}

