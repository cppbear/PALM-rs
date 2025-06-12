// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("test_key".to_string()),
    });

    let result = entry.or_insert_with(|| Value::String("default_value".to_string()));

    assert_eq!(result, &mut Value::String("default_value".to_string()));
    assert_eq!(map.get("test_key"), Some(&Value::String("default_value".to_string())));
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert("existing_key".to_string(), Value::String("existing_value".to_string()));

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.get_mut("existing_key").unwrap(),
    });

    let result = entry.or_insert_with(|| Value::String("default_value".to_string()));

    assert_eq!(result, &mut Value::String("existing_value".to_string()));
    assert_eq!(map.get("existing_key"), Some(&Value::String("existing_value".to_string())));
}

#[test]
#[should_panic]
fn test_or_insert_with_empty_map() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("panicked_key".to_string()),
    });

    let _result = entry.or_insert_with(|| panic!("This should not be called!"));
}

