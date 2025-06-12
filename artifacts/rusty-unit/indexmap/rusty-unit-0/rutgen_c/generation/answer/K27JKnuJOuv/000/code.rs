// Answer 0

#[test]
fn test_get_index_mut2_valid_index() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: IndexSet<i32, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: DummyHasher,
        },
    };

    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    if let Some(value) = index_set.get_index_mut2(0) {
        *value += 10; // modifies value at index 0
    }

    assert_eq!(index_set.map.get(0), Some(&11));
}

#[test]
fn test_get_index_mut2_invalid_index() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: IndexSet<i32, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: DummyHasher,
        },
    };

    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    let result = index_set.get_index_mut2(2); // Trying to access invalid index
    assert!(result.is_none());
}

