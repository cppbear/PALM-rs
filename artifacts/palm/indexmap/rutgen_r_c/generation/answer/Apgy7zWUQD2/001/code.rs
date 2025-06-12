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
    // Assume `insert` is a method that allows us to add elements to the index_set
    for i in 0..5 {
        index_set.map.insert(i, ());
    }

    let new_set = index_set.split_off(3);
    
    assert_eq!(index_set.len(), 3);
    assert_eq!(new_set.len(), 2);
}

#[test]
#[should_panic(expected = "panicked at 'at > len'")] // Adjust the message based on your panic message
fn test_split_off_panics_if_at_greater_than_len() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, TestHasher);
    for i in 0..5 {
        index_set.map.insert(i, ());
    }

    // This should panic as `at` is greater than `len`
    index_set.split_off(6);
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
    let new_set = index_set.split_off(0);
    
    assert!(index_set.is_empty());
    assert!(new_set.is_empty());
}

#[test]
fn test_split_off_exact_capacity() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(10, TestHasher);
    for i in 0..10 {
        index_set.map.insert(i, ());
    }

    let new_set = index_set.split_off(10);
    
    assert_eq!(index_set.len(), 10);
    assert!(new_set.is_empty());
}

