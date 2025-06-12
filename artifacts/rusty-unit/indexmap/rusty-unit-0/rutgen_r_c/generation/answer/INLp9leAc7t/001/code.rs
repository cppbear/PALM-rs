// Answer 0

#[test]
fn test_try_reserve_with_positive_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::<i32, i32, SimpleHasher>::with_capacity_and_hasher(5, SimpleHasher);
    map.try_reserve(3).unwrap();
    assert_eq!(map.capacity(), 5);
}

#[test]
fn test_try_reserve_exceeding_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::<i32, i32, SimpleHasher>::with_capacity_and_hasher(2, SimpleHasher);
    
    assert!(map.try_reserve(usize::MAX).is_err());
}

#[test]
fn test_try_reserve_with_zero_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::<i32, i32, SimpleHasher>::with_capacity_and_hasher(0, SimpleHasher);
    assert!(map.try_reserve(1).is_ok());
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_try_reserve_with_exact_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::<i32, i32, SimpleHasher>::with_capacity_and_hasher(3, SimpleHasher);
    map.try_reserve_exact(3).unwrap();
    assert_eq!(map.capacity(), 3);
} 

#[test]
#[should_panic]
fn test_try_reserve_with_large_exceeding_capacity() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::<i32, i32, SimpleHasher>::with_capacity_and_hasher(1, SimpleHasher);
    map.try_reserve(usize::MAX).unwrap();
}

