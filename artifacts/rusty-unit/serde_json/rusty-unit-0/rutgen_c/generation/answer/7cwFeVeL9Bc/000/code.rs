// Answer 0

#[test]
fn test_hash_with_btree_map() {
    use std::collections::BTreeMap;
    use std::hash::Hasher;

    let mut state = std::collections::hash_map::DefaultHasher::new();
    let mut map_data: BTreeMap<String, Value> = BTreeMap::new();
    
    map_data.insert(String::from("key1"), Value::Number(0.into()));
    map_data.insert(String::from("key2"), Value::String(String::from("value")));

    let map = Map { map: map_data };

    // Invoke hash method
    map.hash(&mut state);
    
    // Verify hash output (checking that it doesn't panic suffices for this context)
}

#[test]
fn test_hash_with_index_map() {
    #[cfg(feature = "preserve_order")]
    {
        use indexmap::IndexMap;
        use std::hash::Hasher;

        let mut state = std::collections::hash_map::DefaultHasher::new();
        let mut map_data: IndexMap<String, Value> = IndexMap::new();

        map_data.insert(String::from("key1"), Value::Number(0.into()));
        map_data.insert(String::from("key2"), Value::String(String::from("value")));

        let map = Map { map: map_data };

        // Invoke hash method
        map.hash(&mut state);
        
        // Verify hash output (checking that it doesn't panic suffices for this context)
    }
}

