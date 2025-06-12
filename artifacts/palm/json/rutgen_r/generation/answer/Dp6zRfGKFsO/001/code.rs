// Answer 0

#[test]
fn test_key_vacant_entry() {
    use serde_json::map::{Map, Entry};

    // Create a new map instance
    let mut map = Map::new();

    // Insert an entry into the map as vacant
    let entry = map.entry("serde");

    // Check if the entry is vacant and fetch the key
    match entry {
        Entry::Vacant(vacant) => {
            // Assert that the key of the vacant entry is as expected
            assert_eq!(vacant.key(), &"serde".to_string());
        }
        Entry::Occupied(_) => {
            panic!("Expected it to be Vacant but found Occupied");
        }
    }
}

#[test]
fn test_key_multiple_entries() {
    use serde_json::map::{Map, Entry};

    // Create a new map instance
    let mut map = Map::new();

    // Insert an entry into the map as vacant
    let entry1 = map.entry("test_key1");
    let entry2 = map.entry("test_key2");

    match entry1 {
        Entry::Vacant(vacant) => {
            // Assert the key for the first vacant entry
            assert_eq!(vacant.key(), &"test_key1".to_string());
            vacant.insert(serde_json::Value::Null); // Insert a value
        }
        Entry::Occupied(_) => {
            panic!("Expected it to be Vacant but found Occupied");
        }
    }

    match entry2 {
        Entry::Vacant(vacant) => {
            // Assert the key for the second vacant entry
            assert_eq!(vacant.key(), &"test_key2".to_string());
            vacant.insert(serde_json::Value::Null); // Insert a value
        }
        Entry::Occupied(_) => {
            panic!("Expected it to be Vacant but found Occupied");
        }
    }
}

