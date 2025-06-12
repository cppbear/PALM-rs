// Answer 0

#[test]
fn test_try_reserve_success() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_map = IndexMap::with_capacity_and_hasher(2, TestHasher);
    assert_eq!(index_map.try_reserve(3), Ok(()));
}

#[test]
fn test_try_reserve_failure() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_map = IndexMap::with_capacity_and_hasher(1, TestHasher);
    index_map.try_reserve(usize::MAX);
    assert_eq!(index_map.try_reserve(usize::MAX), Err(TryReserveError { kind: TryReserveErrorKind::CapacityOverflow }));
}

#[test]
fn test_try_reserve_zero() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_map = IndexMap::with_capacity_and_hasher(0, TestHasher);
    assert_eq!(index_map.try_reserve(0), Ok(()));
}

