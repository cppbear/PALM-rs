// Answer 0

#[test]
fn test_entry_occupied_get() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            assert_eq!(occupied.get(), &json!(12));
        }
        Entry::Vacant(_) => panic!("Expected occupied entry but found vacant."),
    }
}

#[test]
#[should_panic(expected = "Expected occupied entry but found vacant.")]
fn test_entry_vacant_get() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("non_existent_key") {
        Entry::Occupied(occupied) => {
            let _value = occupied.get(); // This should not panic as it is supposed to be vacant.
        }
        Entry::Vacant(_) => {
            panic!("Expected occupied entry but found vacant.");
        }
    }
}

