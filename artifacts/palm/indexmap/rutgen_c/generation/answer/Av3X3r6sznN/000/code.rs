// Answer 0

#[test]
fn test_hasher() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let hash_builder = TestHasher;
    let index_map: IndexMap<i32, &str, TestHasher> = IndexMap::with_capacity_and_hasher(10, hash_builder);
    
    let hasher = index_map.hasher();
    assert!(std::ptr::eq(hasher, &index_map.hash_builder));
}

#[test]
fn test_hasher_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let hash_builder = TestHasher;
    let index_map: IndexMap<i32, &str, TestHasher> = IndexMap::with_hasher(hash_builder);
    
    let hasher = index_map.hasher();
    assert!(std::ptr::eq(hasher, &index_map.hash_builder));
}

