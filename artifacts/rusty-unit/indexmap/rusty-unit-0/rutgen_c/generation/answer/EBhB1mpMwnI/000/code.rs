// Answer 0

#[test]
fn test_reserve_exact_with_capacity() {
    struct TestHashBuilder;
    
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(5, TestHashBuilder);
    map.reserve_exact(3);
    assert_eq!(map.capacity(), 3);
}

#[test]
fn test_reserve_exact_no_initial_capacity() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(0, TestHashBuilder);
    map.reserve_exact(2);
    assert_eq!(map.capacity(), 2);
}

#[test]
fn test_reserve_exact_with_large_capacity() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, TestHashBuilder);
    map.reserve_exact(20);
    assert_eq!(map.capacity(), 20);
}

#[should_panic]
fn test_reserve_exact_panic_negative() {
    struct TestHashBuilder;
    
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(5, TestHashBuilder);
    map.reserve_exact(usize::MAX);
}

