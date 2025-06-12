// Answer 0

#[test]
fn test_entry_key_occupied() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            let key = occupied.key();
            assert_eq!(key, &"serde".to_owned());
        }
        Entry::Vacant(_) => panic!("Expected the entry to be occupied."),
    }
}

#[test]
#[should_panic(expected = "Expected the entry to be occupied.")]
fn test_entry_key_vacant() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("non_existent") {
        Entry::Occupied(_) => panic!("Expected the entry to be vacant."),
        Entry::Vacant(_) => {
            // Expecting panic when we try to access key
            let _key = ""; // This would be a placeholder for actual access that leads to panic
        }
    }
}

