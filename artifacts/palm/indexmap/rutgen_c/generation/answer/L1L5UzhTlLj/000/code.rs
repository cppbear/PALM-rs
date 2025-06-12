// Answer 0

#[test]
fn test_try_reserve_success() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(10, DummyHashBuilder);
    assert_eq!(index_set.try_reserve(5).is_ok(), true);
}

#[test]
fn test_try_reserve_exceed_capacity() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(3, DummyHashBuilder);
    index_set.try_reserve(5).unwrap();
    // Assuming that it can still be reserved even if the initial capacity might be less.
    assert_eq!(index_set.len(), 0);
}

#[test]
#[should_panic]
fn test_try_reserve_negative() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, DummyHashBuilder);
    index_set.try_reserve(usize::MAX).unwrap(); // Expecting to panic due to overflow.
}

#[test]
fn test_try_reserve_zero() {
    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, DummyHashBuilder);
    let result = index_set.try_reserve(0);
    assert!(result.is_ok());
}

