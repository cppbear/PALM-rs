// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestHasher;
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new().build_hasher();
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("poneyland", "existing_value".to_string());

    let raw_entry = map.raw_entry_mut().from_key("poneyland");
    match raw_entry {
        RawEntryMut::Occupied(entry) => {
            entry.insert("new_key".to_string()); // Modifying existing entry
            entry.key(); // Accessing key
            entry.get(); // Accessing value
            let (key, value) = entry.into_key_value(); // Getting mutable references
            *value = "new_value".to_string(); // Modifying the value through mutable reference
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

#[test]
fn test_or_insert_with_empty_entry() {
    struct TestHasher;
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new().build_hasher();
    let mut map: HashMap<&str, String> = HashMap::new();

    let raw_entry = map.raw_entry_mut().from_key("poneyland");
    match raw_entry {
        RawEntryMut::Vacant(entry) => {
            let (key, value) = entry.or_insert_with(|| {
                ("poneyland", "hoho".to_string())
            });
            *value = "new_value".to_string(); // Modifying the value through mutable reference
        },
        RawEntryMut::Occupied(_) => unreachable!(),
    }
}

