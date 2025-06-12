// Answer 0

#[test]
fn test_capacity_non_empty_set() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let set = IndexSet::with_capacity_and_hasher(10, hasher);
    assert_eq!(set.capacity(), 10);
}

#[test]
fn test_capacity_empty_set() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let set: IndexSet<i32, TestHasher> = IndexSet::with_hasher(hasher);
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_capacity_after_reserve() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let mut set = IndexSet::with_capacity_and_hasher(0, hasher);
    set.reserve(20);
    assert!(set.capacity() >= 20);
}

