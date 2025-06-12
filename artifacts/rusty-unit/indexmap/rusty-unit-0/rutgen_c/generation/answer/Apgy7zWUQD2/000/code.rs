// Answer 0

#[test]
fn test_split_off_valid_case() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(10, TestHasher);

    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());

    let split_set = index_set.split_off(2);

    assert_eq!(index_set.len(), 2);
    assert_eq!(split_set.len(), 1);
}

#[test]
#[should_panic]
fn test_split_off_panic_case() {
    let mut index_set = IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());

    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());

    index_set.split_off(4); // This should trigger a panic as the index is out of bounds.
}

#[test]
fn test_split_off_empty_set() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(0, TestHasher);

    let split_set = index_set.split_off(0);

    assert!(index_set.is_empty());
    assert!(split_set.is_empty());
}

