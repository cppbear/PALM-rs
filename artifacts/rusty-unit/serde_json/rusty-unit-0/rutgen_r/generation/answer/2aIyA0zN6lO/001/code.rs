// Answer 0

#[test]
fn test_get_mut_occupied() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();
    map.insert("test".to_owned(), json!([1, 2, 3]));

    match map.entry("test") {
        Entry::Occupied(mut occupied) => {
            let value = occupied.get_mut();
            assert!(value.is_array());
            value.as_array_mut().unwrap().push(json!(4));
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }

    assert_eq!(map["test"].as_array().unwrap().len(), 4);
}

#[test]
#[should_panic]
fn test_get_mut_vacant() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();

    match map.entry("unknown") {
        Entry::Occupied(_) => panic!("Expected entry to be vacant"),
        Entry::Vacant(vacant) => {
            // No action needed for vacant, to simulate panic condition
            // However, the method get_mut does not exist on vacant, so we check panic conditions.
            // This is more just a formality since we're not using get_mut here
        }
    }
}

#[test]
fn test_get_mut_multiple_insertions() {
    use serde_json::json;
    use serde_json::map::Entry;
    use serde_json::Value;

    let mut map = serde_json::Map::new();
    map.insert("data".to_owned(), json!([1, 2]));

    match map.entry("data") {
        Entry::Occupied(mut occupied) => {
            let value = occupied.get_mut();
            assert!(value.is_array());
            value.as_array_mut().unwrap().extend_from_slice(&[json!(3), json!(4)]);
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }

    assert_eq!(map["data"].as_array().unwrap().len(), 4);
}

