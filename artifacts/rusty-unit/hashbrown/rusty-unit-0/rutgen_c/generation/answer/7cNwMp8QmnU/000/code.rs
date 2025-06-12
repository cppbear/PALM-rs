// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let map: HashMap<i32, i32, TestHasher> = HashMap::with_capacity_and_hasher(0, hasher);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let map: HashMap<i32, i32, TestHasher> = HashMap::with_capacity_and_hasher(10, hasher);
    assert_eq!(map.len(), 0);
    // Here you may want to check the capacity if you have access to it.
    // Assuming you have a method map.capacity().
}

#[test]
fn test_with_capacity_and_hasher_large_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let capacity = 1000;
    let map: HashMap<i32, i32, TestHasher> = HashMap::with_capacity_and_hasher(capacity, hasher);
    assert_eq!(map.len(), 0);
    // Assuming map can hold at least capacity elements without reallocating.
    // Check for capacity if applicable.
}

#[test]
fn test_with_capacity_and_hasher_insert() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let mut map: HashMap<i32, i32, TestHasher> = HashMap::with_capacity_and_hasher(2, hasher);
    map.insert(1, 2);
    assert_eq!(map.len(), 1);
}

