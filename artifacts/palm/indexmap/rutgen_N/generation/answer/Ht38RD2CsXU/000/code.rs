// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    struct TestMap<K, V, S> {
        core: TestCore<K, V, S>,
    }

    struct TestCore<K, V, S> {
        entries: Vec<TestEntry<K, V>>,
        indices: TestIndices,
    }

    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    struct TestIndices;

    impl TestIndices {
        fn find_entry<F>(&self, _hash: u64, _eq: F) -> Result<usize, usize>
        where
            F: Fn(&usize) -> bool,
        {
            // Simulate finding an entry
            Ok(0) // Assuming entry is found at index 0
        }
    }

    let entries = vec![TestEntry { key: "test_key", value: "test_value" }];
    let core = TestCore { entries, indices: TestIndices };
    let map = TestMap { core };

    let result = map.from_hash::<_, _>(123, |key| key == &"test_key");
    if let RawEntryMut::Occupied(_) = result {
        // Test is successful
    } else {
        panic!("Expected an occupied entry but got vacant");
    }
}

#[test]
fn test_from_hash_vacant_entry() {
    struct TestMap<K, V, S> {
        core: TestCore<K, V, S>,
        hash_builder: S,
    }

    struct TestCore<K, V, S> {
        entries: Vec<TestEntry<K, V>>,
        indices: TestIndices,
    }

    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    struct TestIndices;

    impl TestIndices {
        fn find_entry<F>(&self, _hash: u64, _eq: F) -> Result<usize, usize>
        where
            F: Fn(&usize) -> bool,
        {
            // Simulate not finding an entry
            Err(usize::MAX) // Indicating no entry was found
        }
    }

    let entries: Vec<TestEntry<&str, &str>> = Vec::new();
    let core = TestCore { entries, indices: TestIndices };
    let map = TestMap { core, hash_builder: () };

    let result = map.from_hash::<_, _>(123, |key| key == &"non_existent_key");
    if let RawEntryMut::Vacant(_) = result {
        // Test is successful
    } else {
        panic!("Expected a vacant entry but got occupied");
    }
}

