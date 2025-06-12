// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    struct DummyHasher; // Custom hasher struct
    impl std::hash::BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, u32, DummyHasher> = HashMap::new();
    map.insert("existing_key".to_string(), 42);

    let entry_ref = EntryRef::Occupied(map.entry_ref("existing_key").unwrap());
    let mut occupied_entry = entry_ref.insert(37);

    assert_eq!(occupied_entry.key(), "existing_key");
    assert_eq!(occupied_entry.get(), &37); // Verify that the value has been updated
}

#[test]
#[should_panic] // This will panic if entry does not exist
fn test_insert_into_non_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    struct DummyHasher; // Custom hasher struct
    impl std::hash::BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, u32, DummyHasher> = HashMap::new();

    let entry_ref = EntryRef::Occupied(map.entry_ref("non_existing_key").unwrap());
    let _ = entry_ref.insert(37); // This should panic, as the entry is not occupied
}

