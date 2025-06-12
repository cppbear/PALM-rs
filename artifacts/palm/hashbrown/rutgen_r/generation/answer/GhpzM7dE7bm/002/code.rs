// Answer 0

#[test]
fn test_try_insert_existing_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::OccupiedError;

    // Initialize a HashMap and insert an initial key-value pair
    let mut map = HashMap::new();
    map.insert(37, "a");

    // Attempt to insert a new value for the existing key
    match map.try_insert(37, "b") {
        Err(OccupiedError { entry, value }) => {
            // Check that the entry corresponds to the key
            assert_eq!(entry.key(), &37);
            // Verify that the original value is still present
            assert_eq!(entry.get(), &"a");
            // Check that the value we attempted to insert is correct
            assert_eq!(value, "b");
        }
        _ => panic!("Expected an Err, but got Ok"),
    }
}

