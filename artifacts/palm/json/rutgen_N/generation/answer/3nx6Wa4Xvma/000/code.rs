// Answer 0

#[test]
fn test_entry_occupied_into_mut() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("serde".to_owned(), json!([1, 2, 3]));

    match map.entry("serde") {
        Entry::Occupied(mut occupied) => {
            occupied.into_mut().as_array_mut().unwrap().push(json!(4));
        }
        Entry::Vacant(_) => unimplemented!(),
    }

    assert_eq!(map["serde"].as_array().unwrap().len(), 4);
}

#[test]
fn test_entry_vacant() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Map;

    let mut map = Map::new();

    match map.entry("serde") {
        Entry::Occupied(_) => panic!("Expected Vacant entry"),
        Entry::Vacant(vacant) => {
            vacant.insert(json!([1, 2, 3]));
        }
    }

    assert_eq!(map["serde"].as_array().unwrap().len(), 3);
}

