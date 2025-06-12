// Answer 0

#[test]
fn test_index_from_hash_found_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestIndexMap {
        core: crate::IndexMapCore<u64, String>, // Using u64 for key with String as value
        indices: Vec<usize>,
    }

    let core = TestIndexMap {
        core: crate::IndexMapCore {
            entries: vec![
                crate::Bucket { hash: HashValue(1), key: 10, value: "value1".to_string() },
                crate::Bucket { hash: HashValue(2), key: 20, value: "value2".to_string() },
            ],
            // This is just a placeholder, you should insert the actual fields of IndexMapCore
        },
        indices: vec![0, 1],
    };

    let map = crate::IndexMap { core, hash_builder: TestHasher };
    let builder = crate::RawEntryBuilder { map: &map };

    // Test for a key that exists
    let result = builder.index_from_hash(1, |key| *key == 10);
    assert_eq!(result, Some(0));

    // Test for a key that doesn't exist
    let result = builder.index_from_hash(1, |key| *key == 30);
    assert_eq!(result, None);
}

#[test]
fn test_index_from_hash_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestIndexMap {
        core: crate::IndexMapCore<u64, String>, 
        indices: Vec<usize>,
    }

    let core = TestIndexMap {
        core: crate::IndexMapCore {
            entries: vec![],
        },
        indices: vec![],
    };

    let map = crate::IndexMap { core, hash_builder: TestHasher };
    let builder = crate::RawEntryBuilder { map: &map };

    // In a completely empty map, any lookup should return None
    let result = builder.index_from_hash(0, |key| *key == 10);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_index_from_hash_panic_condition() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestIndexMap {
        core: crate::IndexMapCore<u64, String>, 
        indices: Vec<usize>,
    }

    let core = TestIndexMap {
        core: crate::IndexMapCore {
            entries: vec![
                crate::Bucket { hash: HashValue(1), key: 10, value: "value1".to_string() },
            ],
            // This is just a placeholder, you should insert the actual fields of IndexMapCore
        },
        indices: vec![0],
    };

    let map = crate::IndexMap { core, hash_builder: TestHasher };
    let builder = crate::RawEntryBuilder { map: &map };

    // Intentionally using a hash that would cause a panic by indexing out of bounds
    let _result = builder.index_from_hash(1, |key| *key == 100);
}

