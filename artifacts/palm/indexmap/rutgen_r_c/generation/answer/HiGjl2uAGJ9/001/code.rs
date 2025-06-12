// Answer 0

#[test]
fn test_from_key_existing_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    struct TestHasher(DefaultHasher);
    
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: IndexMap<String, usize, TestHasher> = IndexMap {
        core: IndexMapCore::new(),
        hash_builder: TestHasher(DefaultHasher::new()),
    };
    
    map.insert("key".to_string(), 42);
    
    let entry_builder = RawEntryBuilderMut { map: &mut map };
    let key = "key";
    let entry = entry_builder.from_key(key);
    
    match entry {
        RawEntryMut::Occupied(_) => {},
        _ => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_from_key_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    struct TestHasher(DefaultHasher);
    
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: IndexMap<String, usize, TestHasher> = IndexMap {
        core: IndexMapCore::new(),
        hash_builder: TestHasher(DefaultHasher::new()),
    };
    
    let entry_builder = RawEntryBuilderMut { map: &mut map };
    let key = "nonexistent_key";
    let entry = entry_builder.from_key(key);
    
    match entry {
        RawEntryMut::Vacant(_) => {},
        _ => panic!("Expected a vacant entry"),
    }
}

