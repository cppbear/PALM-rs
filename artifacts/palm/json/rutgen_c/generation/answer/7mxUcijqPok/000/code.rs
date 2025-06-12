// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use std::collections::BTreeMap;
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();
    map.insert("serde".to_string(), Value::String("cpp".to_string()));

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: OccupiedEntryImpl {
            key: "serde".to_string(),
            value: &mut map["serde"],
        },
    });

    let modified_entry = entry.and_modify(|v| {
        if let Value::String(ref mut s) = v {
            *s = "rust".to_string();
        }
    });

    if let Entry::Occupied(ref occupied) = modified_entry {
        assert_eq!(occupied.get(), &Value::String("rust".to_string()));
    }

    assert_eq!(map["serde"], Value::String("rust".to_string()));
}

#[test]
fn test_and_modify_vacant_entry() {
    use std::collections::BTreeMap;
    use serde_json::{Value, Map};

    let mut map: Map<String, Value> = Map::new();

    let entry = Entry::Vacant(VacantEntry {
        vacant: VacantEntryImpl {
           key: "serde".to_string(),
           map: &mut map,
        },
    });

    let modified_entry = entry.and_modify(|_v| {
        // This block should not be executed since it's a vacant entry.
        panic!("This should not be called");
    });

    if let Entry::Vacant(_) = modified_entry {
        // No assertion needed since we expect the entry to remain vacant.
    }

    map.insert("serde".to_string(), Value::String("cpp".to_string()));
    assert_eq!(map["serde"], Value::String("cpp".to_string()));
}

