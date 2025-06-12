// Answer 0

#[test]
fn test_get_mut_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!([1, 2, 3]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().as_array_mut().unwrap().push(json!(4));
            assert_eq!(occupied.get().as_array().unwrap().len(), 4);
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_get_mut_vacant_entry() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("nonexistent") {
        Entry::Occupied(_) => panic!("Expected entry to be vacant"),
        Entry::Vacant(vacant) => {
            vacant.insert(json!([]));
            assert_eq!(map.get("nonexistent").unwrap().as_array().unwrap().len(), 0);
        }
    }
}

