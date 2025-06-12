// Answer 0

#[test]
fn test_reserve_capacity_increases_length() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(0, TestHasher);
    index_set.reserve(10);
    assert_eq!(index_set.capacity(), 10);
}

#[test]
fn test_reserve_zero_does_not_change_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, TestHasher);
    let initial_capacity = index_set.capacity();
    index_set.reserve(0);
    assert_eq!(index_set.capacity(), initial_capacity);
}

#[test]
fn test_reserve_capacity_increments() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, TestHasher);
    index_set.reserve(5);
    assert!(index_set.capacity() >= 10);
}

#[test]
fn test_reserve_large_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, TestHasher);
    index_set.reserve(1000);
    assert!(index_set.capacity() >= 1000);
}

