// Answer 0

fn test_get_index_of_empty() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let map: IndexMap<i32, i32, MockHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::default(),
            entries: Entries::default(),
        },
        hash_builder: MockHasher,
    };
    
    assert_eq!(map.get_index_of(&1), None);
}

fn test_get_index_of_single_entry() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: MockHasher,
    };
    
    // Insert a single entry
    map.core.entries.push(Bucket {
        hash: HashValue(0),
        key: 1,
        value: 10,
    });
    
    map.core.indices.insert(0, (1, HashValue(0))); // Mocking the index insert

    assert_eq!(map.get_index_of(&1), Some(0));
    assert_eq!(map.get_index_of(&2), None);
}

fn test_get_index_of_multiple_entries() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: MockHasher,
    };

    // Insert multiple entries
    map.core.entries.push(Bucket {
        hash: HashValue(1),
        key: 1,
        value: 10,
    });
    
    map.core.entries.push(Bucket {
        hash: HashValue(2),
        key: 2,
        value: 20,
    });

    map.core.indices.insert(1, (1, HashValue(1))); // Mocking the index insert
    map.core.indices.insert(2, (2, HashValue(2)));

    assert_eq!(map.get_index_of(&1), Some(0));
    assert_eq!(map.get_index_of(&2), Some(1));
    assert_eq!(map.get_index_of(&3), None);
}

