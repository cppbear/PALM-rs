// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    let key = String::from("serde");
    let value = serde_json::Value::String(String::from("existing_value"));
    map.insert(key.clone(), value.clone());

    let entry = serde_json::Entry::Occupied(serde_json::OccupiedEntry {
        occupied: map.get_mut(&key).unwrap(),
    });

    let result = entry.or_insert_with(|| serde_json::Value::String(String::from("hoho")));

    assert_eq!(result, &mut value);
    assert_eq!(map.get(&key), Some(&value));
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    let key = String::from("serde");
    
    let entry = serde_json::Entry::Vacant(serde_json::VacantEntry {
        vacant: map.entry(key.clone()).or_insert(serde_json::Value::Null),
    });

    let result = entry.or_insert_with(|| serde_json::Value::String(String::from("hoho")));

    assert_eq!(result, &mut serde_json::Value::String(String::from("hoho")));
    assert_eq!(map.get(&key), Some(&serde_json::Value::String(String::from("hoho"))));
}

