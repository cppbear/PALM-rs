// Answer 0

#[test]
fn test_entry_occupied() {
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct TestKey(u32);
    
    #[derive(Debug)]
    struct TestValue(String);
    
    struct TestHasher;
    
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.insert(TestKey(1), TestValue("value1".to_string()));
    
    match map.entry(TestKey(1)) {
        Entry::Occupied(_) => (),
        _ => panic!("Expected Occupied entry"),
    }
}

#[test]
fn test_entry_vacant() {
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct TestKey(u32);
    
    #[derive(Debug)]
    struct TestValue(String);
    
    struct TestHasher;
    
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, TestHasher);

    match map.entry(TestKey(2)) {
        Entry::Vacant(_) => (),
        _ => panic!("Expected Vacant entry"),
    }
}

#[test]
fn test_entry_multiple_insertions() {
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct TestKey(u32);
    
    #[derive(Debug)]
    struct TestValue(String);
    
    struct TestHasher;
    
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.insert(TestKey(1), TestValue("initial_value".to_string()));

    map.entry(TestKey(1)).or_insert(TestValue("empty_value".to_string()));
    
    match map.entry(TestKey(1)) {
        Entry::Occupied(entry) => {
            assert_eq!(entry.get().1, &TestValue("initial_value".to_string()));
        },
        _ => panic!("Expected Occupied entry"),
    }
}

