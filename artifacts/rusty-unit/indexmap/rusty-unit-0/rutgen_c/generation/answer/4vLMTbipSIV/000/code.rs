// Answer 0

#[test]
fn test_iter_empty() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let map: IndexMap<i32, i32, TestHasher> = IndexMap::with_capacity_and_hasher(0, TestHasher);
    let iter = map.iter();
    assert_eq!(iter.as_slice().len(), 0);
}

#[test]
fn test_iter_single_element() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::with_capacity_and_hasher(1, TestHasher);
    map.insert(1, 10); 
    let iter = map.iter();
    assert_eq!(iter.as_slice().len(), 1);
    assert_eq!(iter.as_slice()[0].key, 1);
    assert_eq!(iter.as_slice()[0].value, 10);
}

#[test]
fn test_iter_multiple_elements() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::with_capacity_and_hasher(5, TestHasher);
    map.insert(1, 10); 
    map.insert(2, 20);
    map.insert(3, 30);
    
    let iter = map.iter();
    assert_eq!(iter.as_slice().len(), 3);
    assert_eq!(iter.as_slice()[0].key, 1);
    assert_eq!(iter.as_slice()[0].value, 10);
    assert_eq!(iter.as_slice()[1].key, 2);
    assert_eq!(iter.as_slice()[1].value, 20);
    assert_eq!(iter.as_slice()[2].key, 3);
    assert_eq!(iter.as_slice()[2].value, 30);
}

