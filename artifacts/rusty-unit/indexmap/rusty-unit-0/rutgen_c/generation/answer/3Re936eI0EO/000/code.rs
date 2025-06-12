// Answer 0

#[test]
fn test_index_map_debug_fmt() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::FxHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let hasher = TestHasher;
    let mut index_map = IndexMap::with_capacity_and_hasher(10, hasher);
    
    // Assuming that `entries` requires an initial setup of key-value pairs.
    index_map.core.entries.push(Bucket::new("key1".to_string(), "value1".to_string()));
    index_map.core.entries.push(Bucket::new("key2".to_string(), "value2".to_string()));
    
    let debug_str = format!("{:?}", index_map);
    
    assert!(debug_str.contains("key1"));
    assert!(debug_str.contains("value1"));
    assert!(debug_str.contains("key2"));
    assert!(debug_str.contains("value2"));
}

