// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), Value::String("original_value".to_string()));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key1").unwrap() });

    entry.and_modify(|e| *e = Value::String("modified_value".to_string()));
}

#[test]
fn test_and_modify_occupied_entry_multiple_calls() {
    let mut map = serde_json::Map::new();
    map.insert("key2".to_string(), Value::String("first_value".to_string()));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key2").unwrap() });

    entry.and_modify(|e| *e = Value::String("second_value".to_string()));
    entry.and_modify(|e| *e = Value::String("third_value".to_string()));
}

#[test]
fn test_and_modify_occupied_entry_large_map() {
    let mut map = serde_json::Map::new();
    for i in 0..1000 {
        map.insert(format!("key{}", i), Value::String("value".to_string()));
    }
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key500").unwrap() });

    entry.and_modify(|e| *e = Value::String("updated_value".to_string()));
}

#[test]
#[should_panic]
fn test_and_modify_vacant_entry() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry { vacant: map.entry("missing_key") });

    entry.and_modify(|_| {});
}

