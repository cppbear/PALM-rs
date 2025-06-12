// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Test inserting into a vacant entry
    if let Entry::Vacant(o) = map.entry("test_key") {
        let value_ref = o.insert(100);
        assert_eq!(*value_ref, 100);
    }

    // Check that the value is correctly inserted in the map
    assert_eq!(map["test_key"], 100);
}

#[test]
fn test_insert_multiple_vacant_entries() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    let keys = ["first_key", "second_key", "third_key"];
    let values = [10, 20, 30];

    for (key, &value) in keys.iter().zip(&values) {
        if let Entry::Vacant(o) = map.entry(key) {
            let value_ref = o.insert(value);
            assert_eq!(*value_ref, value);
        }
    }

    // Verify all entries were added correctly
    assert_eq!(map["first_key"], 10);
    assert_eq!(map["second_key"], 20);
    assert_eq!(map["third_key"], 30);
}

#[test]
fn test_insert_panic_on_duplicate_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Insert an entry
    if let Entry::Vacant(o) = map.entry("duplicate_key") {
        o.insert(42);
    }

    // Attempt to insert again will not panic but will not invoke VacantEntry
    // No panic expected since it will just return an OccupiedEntry this time
    if let Entry::Occupied(_) = map.entry("duplicate_key") {
        // We can add an assert here to ensure the value wasn't changed
        assert_eq!(map["duplicate_key"], 42); // Verify existing value
    }
}

