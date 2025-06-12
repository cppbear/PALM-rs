// Answer 0

fn test_from_hash_vacant_entry() {
    struct DummyHashBuilder;
    struct DummyKey;
    struct DummyValue;

    let mut entries = Entries::new();
    let mut indices = Indices::new();
    let mut map = IndexMap {
        core: IndexMapCore {
            entries: &mut entries,
            indices: &mut indices,
        },
        hash_builder: DummyHashBuilder,
    };

    let builder = RawEntryBuilderMut { map: &mut map };
    
    let hash: u64 = 1234; // Example hash value
    let is_match = |key: &DummyKey| false; // Always returns false for testing vacant entry

    let result = builder.from_hash(hash, is_match);
    
    match result {
        RawEntryMut::Vacant(vacant) => {
            assert_eq!(vacant.map.entries, &mut entries);
            // Additional assertions can be added as needed
        }
        _ => panic!("Expected a Vacant entry"),
    }
}

fn test_from_hash_occupied_entry() {
    struct DummyHashBuilder;
    struct DummyKey(u64);
    struct DummyValue;

    let mut entries = Entries::new();
    let mut indices = Indices::new();

    // Simulate an occupied entry
    entries.insert(DummyKey(1), DummyValue);
    indices.insert(1, 0); // Assuming this maps the hash to the index of the entry

    let mut map = IndexMap {
        core: IndexMapCore {
            entries: &mut entries,
            indices: &mut indices,
        },
        hash_builder: DummyHashBuilder,
    };

    let builder = RawEntryBuilderMut { map: &mut map };
    
    let hash: u64 = 1; // Assuming this corresponds to the inserted key
    let is_match = |key: &DummyKey| key.0 == 1; // Matches the existing key

    let result = builder.from_hash(hash, is_match);
    
    match result {
        RawEntryMut::Occupied(occupied) => {
            assert_eq!(occupied.entries.lookup(0).unwrap().key, DummyKey(1));
            // Additional assertions can be added as needed
        }
        _ => panic!("Expected an Occupied entry"),
    }
}

