// Answer 0

#[test]
fn test_entry_ref_key_occupied() {
    use hashbrown::HashMap;
    use std::borrow::Cow;

    // Create a HashMap and insert a value
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_owned(), 3);

    // Create an EntryRef from the occupied entry
    let entry_ref = {
        let mut entry_ref = map.entry_ref("poneyland");
        match entry_ref {
            // We can safely assume it's occupied as we inserted above
            EntryRef::Occupied(ref entry) => entry,
            _ => panic!("Expected occupied entry"),
        }
    };

    // Check that the key returns the expected value
    assert_eq!(entry_ref.key(), "poneyland");
}

#[test]
fn test_entry_ref_key_vacant() {
    use hashbrown::HashMap;

    // Create a HashMap without any values
    let mut map: HashMap<String, u32> = HashMap::new();

    // Create an EntryRef from the vacant entry
    let entry_ref = {
        let mut entry_ref = map.entry_ref("horseland");
        match entry_ref {
            EntryRef::Vacant(ref entry) => entry,
            _ => panic!("Expected vacant entry"),
        }
    };

    // Check that the key returns the expected value
    assert_eq!(entry_ref.key(), "horseland");
}

