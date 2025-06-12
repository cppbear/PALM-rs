// Answer 0

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use hashbrown::HashMap;
    use core::hash::BuildHasherDefault;

    // Create a HashMap and insert an initial value
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("example", 42);

    // Access the entry as occupied
    let entry = map.entry("example");
    
    // Use or_insert_with_key to attempt to modify the entry
    let value = entry.or_insert_with_key(|key| {
        // The length of the key should be returned if not occupied
        key.chars().count()
    });

    // Assert the value remains the same and check the value is as expected
    assert_eq!(*value, 42);
}

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;
    use core::hash::BuildHasherDefault;

    // Create a HashMap without any initial values
    let mut map: HashMap<&str, usize> = HashMap::new();

    // Access an entry that is vacant for the key "missing_key"
    let entry = map.entry("missing_key");

    // Using or_insert_with_key to insert a value based on the key
    let value = entry.or_insert_with_key(|key| {
        // Return the count of characters in the key
        key.chars().count()
    });

    // Assert the value was inserted correctly
    assert_eq!(*value, 12);  // "missing_key" has 12 characters
}

#[test]
fn test_or_insert_with_key_existing_entry() {
    use hashbrown::HashMap;
    use core::hash::BuildHasherDefault;

    // Create a HashMap and insert an initial value
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("key1", 30);

    // Access the entry as occupied
    let entry = map.entry("key1");

    // Modify the value using or_insert_with_key
    let value = entry.or_insert_with_key(|key| {
        // The size of the key should trigger an insertion if empty
        key.len() * 2
    });

    // Assert that the value is unchanged and equals the initial value
    assert_eq!(*value, 30);
}

