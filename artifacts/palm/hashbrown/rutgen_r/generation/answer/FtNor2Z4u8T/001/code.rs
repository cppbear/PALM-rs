// Answer 0

#[test]
fn test_get_on_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    match map.entry("poneyland") {
        Entry::Vacant(_) => panic!("Expected entry to be occupied."),
        Entry::Occupied(entry) => {
            let value: &u32 = entry.get();
            assert_eq!(value, &12);
        }
    }
}

#[test]
#[should_panic(expected = "Expected entry to be occupied.")]
fn test_get_on_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.entry("poneyland") {
        Entry::Vacant(_) => {},
        Entry::Occupied(entry) => {
            // This line will never be executed as the entry is vacant.
            let _value: &u32 = entry.get();
        }
    }
}

