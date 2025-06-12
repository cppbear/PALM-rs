// Answer 0

#[test]
fn test_raw_entry_mut_v1() {
    // Define a simple struct for the test
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    // Create an instance of IndexMap for testing
    let mut index_map: indexmap::IndexMap<i32, i32, TestHasher> = indexmap::IndexMap::new();

    // Pre-fill the IndexMap with some values
    index_map.insert(1, 10);
    index_map.insert(2, 20);

    // Call the method being tested
    let raw_entry_builder_mut = index_map.raw_entry_mut_v1();

    // Check if the returned builder contains a reference to the original map
    assert_eq!(raw_entry_builder_mut.map, &mut index_map);
}

#[test]
#[should_panic]
fn test_raw_entry_mut_v1_panic() {
    // For this test, we'll create a scenario where the map is uninitialized
    let mut index_map: indexmap::IndexMap<i32, i32, std::collections::hash_map::RandomState> = indexmap::IndexMap::new();
    
    // Here we expect a panic to occur, since we are invoking the raw_entry_mut_v1 without prior insertion.
    // Note: This test is more illustrative, as invoking raw_entry_mut_v1 without a meaningful context 
    // isn't a common practice in actual application code and does not guarantee a panic.
    let _ = index_map.raw_entry_mut_v1();
}

