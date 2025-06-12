// Answer 0

#[test]
fn test_into_mut() {
    use serde_json::json;
    use serde_json::Map;

    struct TestOccupiedEntry<'a> {
        occupied: OccupiedEntryImpl<'a>,
    }

    impl<'a> OccupiedEntry<'a> {
        fn new(occupied: OccupiedEntryImpl<'a>) -> Self {
            Self { occupied }
        }
    }

    let mut map = Map::new();
    map.insert("test".to_owned(), json!([1, 2, 3]));

    match map.entry("test") {
        Entry::Occupied(mut occupied) => {
            let value = occupied.into_mut();
            value.as_array_mut().unwrap().push(json!(4));
            assert_eq!(value.as_array().unwrap().len(), 4);
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_into_mut_with_other_value() {
    use serde_json::json;
    use serde_json::Map;

    let mut map = Map::new();
    map.insert("test".to_owned(), json!(10));

    match map.entry("test") {
        Entry::Occupied(mut occupied) => {
            let value = occupied.into_mut();
            assert_eq!(value.as_i64().unwrap(), 10);
            *value = json!(20); // Change the value
            assert_eq!(map["test"].as_i64().unwrap(), 20);
        }
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

