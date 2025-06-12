// Answer 0

#[test]
fn test_insert_before_with_valid_index_vacant_entry() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState; // Just for demonstration
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(0, TestHashBuilder);
    let index = map.len();

    // Insert a new key-value pair at the end (index == len)
    let result = map.insert_before(index, '*', ());
    assert_eq!(result, (index, None)); // Expecting (0, None)
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_insert_before_with_out_of_bounds_index() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState; // Just for demonstration
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(0, TestHashBuilder);
    let out_of_bounds_index = 1;

    // This should panic because the map is empty and the index is out of bounds
    map.insert_before(out_of_bounds_index, 'a', ());
}

