// Answer 0

#[test]
fn test_try_reserve_success() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::<u32, TestHasher>::with_capacity_and_hasher(5, TestHasher);
    let result = index_set.try_reserve(3);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_try_reserve_negative() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::<u32, TestHasher>::with_capacity_and_hasher(5, TestHasher);
    index_set.try_reserve(std::usize::MAX);
}

#[test]
fn test_try_reserve_exceed_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::<u32, TestHasher>::with_capacity_and_hasher(2, TestHasher);
    let result = index_set.try_reserve(4);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_no_increment() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::<u32, TestHasher>::with_capacity_and_hasher(5, TestHasher);
    let result = index_set.try_reserve(0);
    assert!(result.is_ok());
}

