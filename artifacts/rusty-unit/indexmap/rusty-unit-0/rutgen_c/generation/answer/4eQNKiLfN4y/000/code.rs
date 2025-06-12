// Answer 0

#[test]
fn test_capacity_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap::with_capacity_and_hasher(0, TestHasher);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_non_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.core.entries.push((1, 10)); // Simulate insertion
    assert!(map.capacity() >= 10); // Test for capacity >= initial
}

#[test]
fn test_capacity_after_clearing() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap::with_capacity_and_hasher(5, TestHasher);
    map.core.entries.push((1, 10)); // Simulate insertion
    map.clear(); // Clear the map
    assert_eq!(map.capacity(), 5); // Capacity should remain the same after clearing
}

#[test]
fn test_capacity_after_shrink_to_fit() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.core.entries.push((1, 10)); // Simulate insertion
    map.shrink_to_fit(); // Shrink to fit
    assert_eq!(map.capacity(), 1); // After shrinking, capacity should equal number of entries
}

