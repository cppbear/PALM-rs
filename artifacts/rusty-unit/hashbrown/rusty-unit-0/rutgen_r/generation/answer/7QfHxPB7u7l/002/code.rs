// Answer 0

#[test]
fn test_key_occupied() {
    use hashbrown::HashMap;
    use std::borrow::Borrow;

    // Setup a HashMap and insert an entry
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 3);

    // Get a reference to the entry
    let entry_ref = map.entry_ref("poneyland");

    // Verify the key() method works for an occupied entry
    assert_eq!(entry_ref.key(), "poneyland");
}

#[test]
fn test_key_vacant() {
    use hashbrown::HashMap;
    use std::borrow::Borrow;

    // Setup a HashMap without inserting any entries
    let mut map: HashMap<String, u32> = HashMap::new();

    // Get a reference to a vacant entry
    let entry_ref = map.entry_ref("horseland");

    // Verify the key() method works for a vacant entry
    assert_eq!(entry_ref.key(), "horseland");
}

#[should_panic]
fn test_key_panic_occupied_invalid() {
    use hashbrown::HashMap;

    // Setup a HashMap and insert an entry
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 3);

    // Get a reference to a vacant entry
    let entry_ref = map.entry_ref("horseland");

    // Attempt to call key() on an occupied entry which shouldn't panic
    entry_ref.key(); // Expect no panic here
}

