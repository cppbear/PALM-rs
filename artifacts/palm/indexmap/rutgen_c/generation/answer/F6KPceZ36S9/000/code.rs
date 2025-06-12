// Answer 0

#[test]
fn test_reserve_exact_initial_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, SimpleHasher> = super::IndexSet::with_capacity_and_hasher(0, SimpleHasher);
    
    index_set.reserve_exact(5);
    
    assert_eq!(index_set.capacity(), 5);
}

#[test]
fn test_reserve_exact_increase_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, SimpleHasher> = super::IndexSet::with_capacity_and_hasher(5, SimpleHasher);

    index_set.reserve_exact(3);
    
    assert_eq!(index_set.capacity(), 5); // Capacity does not change with additional reserve if it's not exceeded

    index_set.reserve_exact(10);
    
    assert_eq!(index_set.capacity(), 10);
}

#[test]
fn test_reserve_exact_zero_additional_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, SimpleHasher> = super::IndexSet::with_capacity_and_hasher(5, SimpleHasher);
    
    let original_capacity = index_set.capacity();
    index_set.reserve_exact(0);
    
    assert_eq!(index_set.capacity(), original_capacity);
}

#[should_panic]
fn test_reserve_exact_exceeding_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, SimpleHasher> = super::IndexSet::with_capacity_and_hasher(2, SimpleHasher);
    
    index_set.reserve_exact(3); // This assumes a panic or issue on over-allocating, specifics depend on the implementation
}

