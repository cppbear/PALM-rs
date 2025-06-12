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
    
    let index_set: super::IndexSet<i32, TestHasher> = super::IndexSet::with_capacity_and_hasher(0, TestHasher);
    let iter = index_set.iter();
    
    assert_eq!(iter.as_slice().len(), 0);
}

#[test]
fn test_iter_non_empty() {
    struct TestHasher;
    
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet::with_capacity_and_hasher(10, TestHasher);
    index_set.reserve(3);
    // Simulating entries as the IndexSet does not have insert logic in the context
    let entries = vec![
        super::Bucket { hash: 1, key: 10, value: () },
        super::Bucket { hash: 2, key: 20, value: () },
        super::Bucket { hash: 3, key: 30, value: () },
    ];
    
    // Assume there's a way to manipulate internal state to add entries for testing.
    index_set.map.core = super::IndexMapCore { entries };

    let iter = index_set.iter();
    
    assert_eq!(iter.as_slice().len(), 3);
}

#[test]
fn test_iter_after_clear() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet::with_capacity_and_hasher(5, TestHasher);
    
    index_set.reserve(5);
    // Simulating entries as the IndexSet does not have insert logic in the context
    let entries = vec![
        super::Bucket { hash: 1, key: 10, value: () },
        super::Bucket { hash: 2, key: 20, value: () },
    ];
    
    // Assume there's a way to manipulate internal state to add entries for testing.
    index_set.map.core = super::IndexMapCore { entries };

    index_set.clear();
    let iter = index_set.iter();
    
    assert_eq!(iter.as_slice().len(), 0);
}

