// Answer 0

#[test]
fn test_try_insert_with_occupied_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::OccupiedError;

    let mut map = HashMap::new();
    
    // Initial insertion
    assert_eq!(map.try_insert(42, "initial").unwrap(), &"initial");

    // Attempt to insert a new value with the same key
    match map.try_insert(42, "new_value") {
        Err(OccupiedError { entry, value }) => {
            // Check that the key is the same as the occupied entry
            assert_eq!(entry.key(), &42);
            // Check that the value corresponding to the occupied entry is what was previously inserted
            assert_eq!(entry.get(), &"initial");
            // The value being inserted should be "new_value"
            assert_eq!(value, "new_value");
        }
        _ => panic!("Expected an Err variant but got Ok"),
    }
}

#[test]
fn test_try_insert_multiple_entries() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::OccupiedError;

    let mut map = HashMap::new();
    
    // First entry
    assert_eq!(map.try_insert(100, "value1").unwrap(), &"value1");

    // Overwriting the first entry
    match map.try_insert(100, "value2") {
        Err(OccupiedError { entry, value }) => {
            assert_eq!(entry.key(), &100);
            assert_eq!(entry.get(), &"value1");
            assert_eq!(value, "value2");
        }
        _ => panic!("Expected an Err variant but got Ok"),
    }

    // Insert a new unique key
    assert_eq!(map.try_insert(200, "value3").unwrap(), &"value3");

    // Attempt to overwrite the second entry
    match map.try_insert(200, "value4") {
        Err(OccupiedError { entry, value }) => {
            assert_eq!(entry.key(), &200);
            assert_eq!(entry.get(), &"value3");
            assert_eq!(value, "value4");
        }
        _ => panic!("Expected an Err variant but got Ok"),
    }
}

