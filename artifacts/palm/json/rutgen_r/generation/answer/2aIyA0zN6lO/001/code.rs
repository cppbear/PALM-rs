// Answer 0

#[test]
fn test_entry_get_mut_occupied() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();
    map.insert("key".to_owned(), json!([1, 2, 3]));

    match map.entry("key") {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().as_array_mut().unwrap().push(json!(4));
            assert_eq!(map["key"].as_array().unwrap().len(), 4);
        }
        Entry::Vacant(_) => panic!("Entry should be occupied but is vacant"),
    }
}

#[test]
#[should_panic(expected = "panicking because entry is vacant")]
fn test_entry_get_mut_vacant() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();

    match map.entry("non_existing_key") {
        Entry::Occupied(_) => panic!("Entry should be vacant but is occupied"),
        Entry::Vacant(mut vacant) => {
            // This will not panic but demonstrates handling of a vacant entry
            if let Some(_) = vacant.insert(json!([1, 2])) {
                panic!("Vacant entry should not have an existing value");
            }
        }
    }
}

