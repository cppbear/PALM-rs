// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::new(); // Assuming there's a method to create a new IndexMap
    map.insert("key1", "value1"); // Assuming there's an insert method
    let mut builder = RawEntryBuilderMut { map: &mut map };
    
    let entry = builder.from_hash(0, |key: &str| key == "key1");
    
    match entry {
        RawEntryMut::Occupied(_) => {},
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_from_hash_vacant_entry() {
    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map = IndexMap::new(); // Assuming there's a method to create a new IndexMap
    let mut builder = RawEntryBuilderMut { map: &mut map };
    
    let entry = builder.from_hash(0, |key: &str| key == "nonexistent_key");
    
    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!("Expected a vacant entry"),
    }
}

