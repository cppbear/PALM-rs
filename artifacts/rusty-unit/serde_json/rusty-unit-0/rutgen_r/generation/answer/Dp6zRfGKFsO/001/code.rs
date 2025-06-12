// Answer 0

#[test]
fn test_key_on_vacant_entry() {
    use serde_json::map::{Map, Entry};

    let mut map = Map::new();

    match map.entry("serde") {
        Entry::Vacant(vacant) => {
            assert_eq!(vacant.key(), &"serde".to_string());
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry, got Occupied."),
    }
}

#[test]
fn test_key_with_empty_map() {
    use serde_json::map::{Map, Entry};

    let mut map = Map::new();

    match map.entry("empty") {
        Entry::Vacant(vacant) => {
            assert_eq!(vacant.key(), &"empty".to_string());
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry, got Occupied."),
    }
}

#[test]
fn test_key_with_non_string_key() {
    use serde_json::map::{Map, Entry};

    let mut map = Map::new();

    match map.entry("non_string_key") {
        Entry::Vacant(vacant) => {
            assert_eq!(vacant.key(), &"non_string_key".to_string());
        }
        Entry::Occupied(_) => panic!("Expected Vacant entry, got Occupied."),
    }
}

