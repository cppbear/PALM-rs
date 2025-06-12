// Answer 0

#[test]
fn test_from_hash_full_index_out_of_bounds() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = super::IndexMap::new(TestHasher);

    // Fill the map with one entry
    map.insert("a", 1);

    let raw_entry_builder = super::RawEntryBuilder { map: &map };
    
    // Using a hash that doesn't correspond to the existing entry
    let result = raw_entry_builder.from_hash_full(999, |key| key == &"a");
    
    // Expect None because the computed index would be out of bounds
    assert_eq!(result, None);
}

#[test]
fn test_from_hash_full_match_index_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = super::IndexMap::new(TestHasher);

    // Populate map with an entry
    map.insert("b", 2);

    let raw_entry_builder = super::RawEntryBuilder { map: &map };
    
    // Assuming that the hash of "b" is known
    let hash = 2; // Simulate the correct hash for the key "b"

    let result = raw_entry_builder.from_hash_full(hash, |key| key == &"b");

    // Check that we successfully get the index and references to key and value
    assert!(result.is_some());
    let (index, key, value) = result.unwrap();
    assert_eq!(index, 0);
    assert_eq!(key, &"b");
    assert_eq!(*value, 2);
}

