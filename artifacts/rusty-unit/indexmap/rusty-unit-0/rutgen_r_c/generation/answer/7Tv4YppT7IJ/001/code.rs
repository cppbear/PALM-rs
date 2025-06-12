// Answer 0

#[test]
fn test_keys_empty_map() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(0, DummyHashBuilder);
    let keys = map.keys();
    assert_eq!(keys.iter.len(), 0);
}

#[test]
fn test_keys_non_empty_map() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(10, DummyHashBuilder);
    map.insert(1, 10);
    map.insert(2, 20);
    
    let keys = map.keys();
    let mut collected_keys: Vec<_> = keys.iter.collect();
    collected_keys.sort();
    assert_eq!(collected_keys, vec![1, 2]);
}

#[test]
fn test_keys_large_map() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, i32, DummyHashBuilder> = IndexMap::with_capacity_and_hasher(100, DummyHashBuilder);
    for i in 0..100 {
        map.insert(i, i * 10);
    }
    
    let keys = map.keys();
    let mut collected_keys: Vec<_> = keys.iter.collect();
    collected_keys.sort();
    assert_eq!(collected_keys, (0..100).collect::<Vec<_>>());
}

