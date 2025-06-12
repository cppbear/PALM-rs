// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero() {
    struct DummyHasher;
    
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let n = 0;
    let hasher = DummyHasher;
    
    let map: IndexMap<i32, i32, DummyHasher> = IndexMap::with_capacity_and_hasher(n, hasher);
    
    assert_eq!(map.capacity(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_and_hasher_non_zero() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let n = 5;
    let hasher = DummyHasher;
    
    let map: IndexMap<i32, i32, DummyHasher> = IndexMap::with_capacity_and_hasher(n, hasher);
    
    assert_eq!(map.capacity(), n);
    assert!(map.is_empty());
}

