// Answer 0

#[test]
fn test_with_capacity_and_hasher_non_zero() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let map: IndexMap<i32, i32, TestHasher> = IndexMap::with_capacity_and_hasher(10, TestHasher);
    assert_eq!(map.capacity(), 10);
}

#[test]
fn test_with_capacity_and_hasher_zero() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let map: IndexMap<i32, i32, TestHasher> = IndexMap::with_capacity_and_hasher(0, TestHasher);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_and_hasher_large_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let map: IndexMap<i32, i32, TestHasher> = IndexMap::with_capacity_and_hasher(1000, TestHasher);
    assert_eq!(map.capacity(), 1000);
}

