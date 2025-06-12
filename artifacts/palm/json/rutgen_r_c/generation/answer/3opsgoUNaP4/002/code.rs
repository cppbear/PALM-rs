// Answer 0

#[test]
fn test_key_vacant_entry() {
    use std::collections::BTreeMap;
    use serde_json::Value;

    // Create a BTreeMap to mimic the MapImpl in the context.
    let mut map: BTreeMap<String, Value> = BTreeMap::new();

    // Create a VacantEntry since there is no entry for the key "serde".
    let key = String::from("serde");
    let vacant_entry = VacantEntry { vacant: map.entry(key.clone()) };

    // Create an Entry enum instance with the VacantEntry.
    let entry = Entry::Vacant(vacant_entry);

    // Expect the key returned from the entry to match the input key.
    assert_eq!(entry.key(), &key);
}

#[test]
fn test_key_occupied_entry() {
    use std::collections::BTreeMap;
    use serde_json::Value;

    // Create a BTreeMap to mimic the MapImpl in the context.
    let mut map: BTreeMap<String, Value> = BTreeMap::new();
    let key = String::from("serde");
    let value = Value::String(String::from("example"));

    // Insert an entry to create an OccupiedEntry.
    map.insert(key.clone(), value);

    // Create an OccupiedEntry from the existing map.
    let occupied_entry = OccupiedEntry { occupied: map.entry(key.clone()).or_insert(Value::Null) };

    // Create an Entry enum instance with the OccupiedEntry.
    let entry = Entry::Occupied(occupied_entry);

    // Expect the key returned from the entry to match the key used for insertion.
    assert_eq!(entry.key(), &key);
}

