// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    struct DummyHasher;

    impl std::hash::BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hash_builder = DummyHasher;
    let map: IndexMap<_, _> = with_capacity_and_hasher(0, hash_builder);
    
    assert_eq!(map.core.capacity(), 0);
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    struct DummyHasher;

    impl std::hash::BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hash_builder = DummyHasher;
    let n = 5;
    let map: IndexMap<_, _> = with_capacity_and_hasher(n, hash_builder);
    
    assert_eq!(map.core.capacity(), n);
}

