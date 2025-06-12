// Answer 0

#[test]
fn test_split_off_valid_range() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(10, TestHasher);
    for i in 0..5 {
        map.core.entries.push((i, i * 2)); // Key-value pairs (0,0), (1,2), (2,4), (3,6), (4,8)
    }

    let split_map = map.split_off(3);
    assert_eq!(map.core.entries.len(), 3);
    assert_eq!(split_map.core.entries.len(), 2);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 6")]
fn test_split_off_panic_out_of_bounds() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(5, TestHasher);
    for i in 0..5 {
        map.core.entries.push((i, i * 2));
    }

    // This should panic as 6 is greater than the length of the map
    let _ = map.split_off(6);
}

#[test]
fn test_split_off_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(0, TestHasher);
    let split_map = map.split_off(0);
    assert!(map.core.entries.is_empty());
    assert!(split_map.core.entries.is_empty());
}

#[test]
fn test_split_off_full_capacity() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::with_capacity_and_hasher(5, TestHasher);
    for i in 0..5 {
        map.core.entries.push((i, i * 2));
    }

    // Split the map at the length, which should return an empty map
    let split_map = map.split_off(5);
    assert_eq!(map.core.entries.len(), 5);
    assert!(split_map.core.entries.is_empty());
}

