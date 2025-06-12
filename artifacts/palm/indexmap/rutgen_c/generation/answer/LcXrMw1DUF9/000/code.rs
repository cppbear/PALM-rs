// Answer 0

#[test]
fn test_try_reserve_exact_with_additional_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::RustcHash;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_map: crate::IndexMap<i32, i32, TestHasher> = crate::IndexMap::with_capacity_and_hasher(5, TestHasher);
    
    let result = index_map.try_reserve_exact(3);
    assert!(result.is_ok());
    assert_eq!(index_map.core.capacity(), 5); // ensuring capacity does not exceed initialized
}

#[test]
fn test_try_reserve_exact_with_zero_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::RustcHash;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_map: crate::IndexMap<i32, i32, TestHasher> = crate::IndexMap::with_capacity_and_hasher(0, TestHasher);
    
    let result = index_map.try_reserve_exact(0);
    assert!(result.is_ok());
    assert_eq!(index_map.core.capacity(), 0); // ensuring capacity remains 0
}

#[test]
#[should_panic] // Assuming a panic occurs on exceeding capacity
fn test_try_reserve_exact_exceeding_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::RustcHash;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_map: crate::IndexMap<i32, i32, TestHasher> = crate::IndexMap::with_capacity_and_hasher(2, TestHasher);
    let _ = index_map.try_reserve_exact(usize::MAX); // This is expected to panic
}

