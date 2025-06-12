// Answer 0

#[test]
fn test_values() {
    struct TestHashBuilder;
    
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, &str, TestHashBuilder> = IndexMap::with_capacity_and_hasher(10, TestHashBuilder);
    
    // Simulated insertion of elements (this is assumed as the actual insertion methods are not provided)
    index_map.core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: "one" });
    index_map.core.entries.push(Bucket { hash: HashValue::default(), key: 2, value: "two" });
    
    let values: Vec<_> = index_map.values().iter.collect();
    
    assert_eq!(values, vec!["one", "two"]);
}

#[test]
fn test_values_empty() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let index_map: IndexMap<i32, &str, TestHashBuilder> = IndexMap::with_capacity_and_hasher(0, TestHashBuilder);
    
    let values: Vec<_> = index_map.values().iter.collect();
    
    assert_eq!(values.len(), 0);
}

