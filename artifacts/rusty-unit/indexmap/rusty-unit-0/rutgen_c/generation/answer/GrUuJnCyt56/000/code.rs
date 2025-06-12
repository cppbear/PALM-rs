// Answer 0

#[test]
fn test_clear() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, MockHasher> = super::IndexSet::with_capacity_and_hasher(10, MockHasher);

    // Assuming we have a way to add elements, let's add a few elements.
    // Here we will simulate adding elements to an internal structure.
    index_set.clear(); // Clear the set

    // Check that the set is now empty
    assert!(index_set.is_empty());
}

#[test]
fn test_clear_preserves_capacity() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, MockHasher> = super::IndexSet::with_capacity_and_hasher(10, MockHasher);
    
    // Clear the set
    index_set.clear();

    // Check that the capacity is still the same as before
    assert_eq!(index_set.capacity(), 10);
}

#[test]
fn test_clear_on_empty_set() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: super::IndexSet<i32, MockHasher> = super::IndexSet::with_capacity_and_hasher(10, MockHasher);
    
    // Clear an empty set
    index_set.clear();

    // Check that the capacity is still the same and the set is still empty
    assert!(index_set.is_empty());
    assert_eq!(index_set.capacity(), 10);
}

