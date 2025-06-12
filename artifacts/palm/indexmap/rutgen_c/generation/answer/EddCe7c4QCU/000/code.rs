// Answer 0

#[test]
fn test_iter_non_empty_set() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(10, TestHasher);
    index_set.map.core.insert(1, ());
    index_set.map.core.insert(2, ());
    
    let mut iter = index_set.iter();
    assert_eq!(iter.as_slice().len(), 2);
}

#[test]
fn test_iter_empty_set() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(10, TestHasher);
    
    let iter = index_set.iter();
    assert_eq!(iter.as_slice().len(), 0);
}

