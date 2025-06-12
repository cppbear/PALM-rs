// Answer 0

#[test]
fn test_from_key_hashed_nocheck_vacant_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    impl Hasher for TestHasher {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 {
            42 // Fixed hash for testing
        }
    }

    let mut index_map: IndexMap<i32, i32, TestHasher> = IndexMap {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };

    let builder = RawEntryBuilderMut { map: &mut index_map };
    let entry = builder.from_key_hashed_nocheck(42, &5);

    if let RawEntryMut::Vacant(_) = entry {
        // Test that we received a Vacant entry
    } else {
        panic!("Expected a Vacant entry");
    }
}

#[test]
fn test_from_key_hashed_nocheck_occupied_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    impl Hasher for TestHasher {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 {
            42 // Fixed hash for testing
        }
    }

    let mut index_map: IndexMap<i32, i32, TestHasher> = IndexMap {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };

    index_map.insert(5, 100); // Pre-populate the map
    let builder = RawEntryBuilderMut { map: &mut index_map };
    let entry = builder.from_key_hashed_nocheck(42, &5);

    if let RawEntryMut::Occupied(_) = entry {
        // Test that we received an Occupied entry
    } else {
        panic!("Expected an Occupied entry");
    }
}

