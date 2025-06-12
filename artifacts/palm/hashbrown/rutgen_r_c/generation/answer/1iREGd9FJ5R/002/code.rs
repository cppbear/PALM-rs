// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::hash::{BuildHasher, Hasher};

    struct TestHasher;
    impl BuildHasher for TestHasher {
        fn build_hasher(&self) -> std::collections::hash_map::DefaultHasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    // Get the RawEntryMut which is guaranteed to be Occupied because of the insertions above
    let entry = map.raw_entry_mut().from_key("key1");

    // Perform insert operation which should return the RawOccupiedEntryMut
    let occupied_entry = entry.insert("key1", 5);

    // Verify the inserted value is returned properly and the value is updated
    assert_eq!(occupied_entry.get(), &5);
    assert_eq!(occupied_entry.get_key_value(), (&"key1", &5));
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map.raw_entry_mut().from_key("key3");

    // Insert into the vacant entry which is expected to act as if it is newly inserted
    let occupied_entry = entry.insert("key3", 10);

    // Verify that a new entry was created and returned
    assert_eq!(occupied_entry.get(), &10);
    assert_eq!(occupied_entry.get_key_value(), (&"key3", &10));
}

