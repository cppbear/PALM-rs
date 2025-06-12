// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key"),
    });
    
    let value_ref = entry.or_insert_with(|| Value::String("default".to_string()));
    assert_eq!(*value_ref, Value::String("default".to_string()));
    assert_eq!(map.get("key").unwrap(), &Value::String("default".to_string()));
}

#[test]
fn test_or_insert_with_occupied_entry() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_string(), Value::String("existing".to_string()));
    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.entry("key").occupied(),
    });
    
    let value_ref = entry.or_insert_with(|| Value::String("default".to_string()));
    assert_eq!(*value_ref, Value::String("existing".to_string()));
    assert_eq!(map.get("key").unwrap(), &Value::String("existing".to_string()));
}

