// Answer 0

#[test]
fn test_truncate_with_smaller_length() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map = IndexMap::<i32, i32, MockHasher>::with_capacity_and_hasher(10, MockHasher);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    map.truncate(2);
    
    assert_eq!(map.len(), 2);
}

#[test]
fn test_truncate_with_equal_length() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map = IndexMap::<i32, i32, MockHasher>::with_capacity_and_hasher(5, MockHasher);
    map.insert(1, 10);
    map.insert(2, 20);
    
    map.truncate(2);
    
    assert_eq!(map.len(), 2);
}

#[test]
fn test_truncate_with_greater_length() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map = IndexMap::<i32, i32, MockHasher>::with_capacity_and_hasher(5, MockHasher);
    map.insert(1, 10);
    map.insert(2, 20);
    
    map.truncate(5);
    
    assert_eq!(map.len(), 2);
}

#[test]
fn test_truncate_empty_map() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher { std::collections::hash_map::RandomState::new() }
    }

    let mut map = IndexMap::<i32, i32, MockHasher>::with_capacity_and_hasher(0, MockHasher);
    
    map.truncate(1);
    
    assert_eq!(map.len(), 0);
}

