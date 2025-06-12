// Answer 0

#[test]
fn test_get_mut_existing_key() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<String, i32, MockHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: MockHasher,
    };

    index_map.insert("key1".to_string(), 10);
    index_map.insert("key2".to_string(), 20);

    let value_ref: Option<&mut i32> = index_map.get_mut(&"key1".to_string());

    assert!(value_ref.is_some());
    assert_eq!(*value_ref.unwrap(), 10);
}

#[test]
fn test_get_mut_non_existing_key() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<String, i32, MockHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: MockHasher,
    };

    index_map.insert("key1".to_string(), 10);
    index_map.insert("key2".to_string(), 20);

    let value_ref: Option<&mut i32> = index_map.get_mut(&"nonexistent_key".to_string());

    assert!(value_ref.is_none());
}

