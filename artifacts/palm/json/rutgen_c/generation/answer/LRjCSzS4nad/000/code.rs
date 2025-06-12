// Answer 0

#[test]
fn test_occupied_entry_key() {
    use std::collections::BTreeMap;
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();
    map.insert("serde".to_owned(), Value::Number(12.into()));

    match map.entry("serde".to_owned()) {
        serde_json::map::Entry::Occupied(occupied) => {
            assert_eq!(occupied.key(), &"serde".to_owned());
        }
        serde_json::map::Entry::Vacant(_) => {
            panic!("Entry should be occupied");
        }
    }
}

#[test]
fn test_occupied_entry_key_with_empty_map() {
    use std::collections::BTreeMap;
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();

    match map.entry("serde".to_owned()) {
        serde_json::map::Entry::Occupied(_) => {
            panic!("Entry should be vacant");
        }
        serde_json::map::Entry::Vacant(vacant) => {
            vacant.insert(Value::Number(12.into()));
            // Verify the key of the newly inserted entry
            let occupied_entry = map.entry("serde".to_owned());
            match occupied_entry {
                serde_json::map::Entry::Occupied(occupied) => {
                    assert_eq!(occupied.key(), &"serde".to_owned());
                }
                serde_json::map::Entry::Vacant(_) => {
                    panic!("Entry should be occupied now");
                }
            }
        }
    }
}

