// Answer 0

#[test]
fn test_swap_remove_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u32, String, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHasher,
    };

    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());

    let result = map.swap_remove(&1);
    assert_eq!(result, Some("value1".to_string()));
    assert!(map.get(&1).is_none());
    assert_eq!(map.get(&2), Some(&"value2".to_string()));
}

#[test]
fn test_swap_remove_non_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u32, String, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHasher,
    };

    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());

    let result = map.swap_remove(&3);
    assert_eq!(result, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_swap_remove_last_element() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u32, String, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHasher,
    };

    map.insert(1, "value1".to_string());

    let result = map.swap_remove(&1);
    assert_eq!(result, Some("value1".to_string()));
    assert!(map.get(&1).is_none());
}

#[test]
#[should_panic]
fn test_swap_remove_panic() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u32, String, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHasher,
    };

    map.swap_remove(&1); // panic occurs since key does not exist
}

