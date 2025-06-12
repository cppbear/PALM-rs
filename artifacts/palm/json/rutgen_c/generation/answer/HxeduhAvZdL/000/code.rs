// Answer 0

#[test]
fn test_insert_existing_entry() {
    use serde_json::json;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("key".to_owned(), json!(10));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.insert(json!(20)), json!(10));
            assert_eq!(occupied.get(), &json!(20));
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_insert_new_entry() {
    use serde_json::json;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("new_key") {
        Entry::Occupied(_) => panic!("Expected vacant entry"),
        Entry::Vacant(vacant) => {
            assert_eq!(vacant.insert(json!(30)), json!(null));
            assert_eq!(vacant.get(), &json!(30));
        }
    }
}

#[test]
fn test_insert_with_removal() {
    use serde_json::json;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("remove_key".to_owned(), json!(5));

    match map.entry("remove_key") {
        Entry::Occupied(mut occupied) => {
            assert_eq!(occupied.remove(), json!(5));
            assert!(map.get("remove_key").is_none());
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

