// Answer 0

#[test]
fn test_key_for_occupied_entry() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();
    map.insert("serde".to_owned(), json!(12));

    match map.entry("serde") {
        Entry::Occupied(occupied) => {
            assert_eq!(occupied.key(), &"serde".to_owned());
        }
        Entry::Vacant(_) => panic!("Expected entry to be occupied"),
    }
}

#[test]
fn test_key_for_vacant_entry() {
    use serde_json::json;
    use serde_json::map::Entry;

    let mut map = serde_json::Map::new();

    match map.entry("not_present") {
        Entry::Occupied(_) => panic!("Expected entry to be vacant"),
        Entry::Vacant(vacant) => {
            assert_eq!(vacant.key(), &"not_present".to_owned());
        }
    }
}

