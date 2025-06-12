// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use serde_json::{json, Value, Map};
    use serde_json::map::Entry;

    let mut map = Map::new();
    let key = "serde";
    let value = json!("hoho");

    match map.entry(key) {
        Entry::Vacant(vacant) => {
            let mut_ref = vacant.insert(value);
            assert_eq!(*mut_ref, json!("hoho"));
            assert!(map.contains_key(key));
        }
        Entry::Occupied(_) => panic!("Expected a VacantEntry, got OccupiedEntry"),
    }
}

#[test]
fn test_insert_overwrite_occupied_entry() {
    use serde_json::{json, Value, Map};
    use serde_json::map::Entry;

    let mut map = Map::new();
    let key = "serde";
    let initial_value = json!("initial");
    map.insert(key.to_string(), initial_value);

    match map.entry(key) {
        Entry::Vacant(_) => panic!("Expected an OccupiedEntry, got VacantEntry"),
        Entry::Occupied(mut occupied) => {
            let mut_ref = occupied.insert(json!("updated"));
            assert_eq!(*mut_ref, json!("updated"));
            assert_eq!(map[key], json!("updated"));
        }
    }
}

