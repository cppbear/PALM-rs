// Answer 0

#[test]
fn test_insert_full_new_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: Default::default(),
            hash_builder: TestHasher,
        },
    };

    let (index, exists) = index_set.insert_full(42);
    assert_eq!(index, 0);
    assert!(exists);
}

#[test]
fn test_insert_full_existing_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: Default::default(),
            hash_builder: TestHasher,
        },
    };

    index_set.insert_full(42);
    let (index, exists) = index_set.insert_full(42);
    assert_eq!(index, 0);  
    assert!(!exists);  
}

#[test]
fn test_insert_full_multiple_values() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: Default::default(),
            hash_builder: TestHasher,
        },
    };

    let (index1, exists1) = index_set.insert_full(42);
    let (index2, exists2) = index_set.insert_full(43);
    
    assert_eq!(index1, 0);
    assert!(exists1);

    assert_eq!(index2, 1);
    assert!(exists2);
    
    let (index3, exists3) = index_set.insert_full(42);
    assert_eq!(index3, 0);
    assert!(!exists3);
}

