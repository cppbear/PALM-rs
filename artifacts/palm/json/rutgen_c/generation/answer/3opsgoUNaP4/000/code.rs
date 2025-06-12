// Answer 0

#[test]
fn test_key_vacant_entry() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    let vacant_entry = Entry::Vacant(VacantEntry {
        vacant: map.entry(String::from("serde")),
    });
    assert_eq!(vacant_entry.key(), &"serde".to_string());
}

#[test]
fn test_key_occupied_entry() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    map.insert(String::from("serde"), Value::String(String::from("json")));
    let occupied_entry = Entry::Occupied(OccupiedEntry {
        occupied: map.entry(String::from("serde")).or_insert(Value::Null),
    });
    assert_eq!(occupied_entry.key(), &"serde".to_string());
}

