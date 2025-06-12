// Answer 0

#[test]
fn test_from_hash_with_matching_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    let builder = RawEntryBuilder { map: &map};

    let hash = {
        let mut hasher = TestHasher.build_hasher();
        "key1".hash(&mut hasher);
        hasher.finish()
    };
    
    let result = builder.from_hash(hash, |k| k == &"key1");
    assert_eq!(result, Some((&"key1", &"value1")));
}

#[test]
fn test_from_hash_with_no_matching_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    
    let builder = RawEntryBuilder { map: &map };

    let hash = {
        let mut hasher = TestHasher.build_hasher();
        "key1".hash(&mut hasher);
        hasher.finish()
    };

    let result = builder.from_hash(hash, |k| k == &"nonexistent_key");
    assert_eq!(result, None);
}

#[test]
fn test_from_hash_with_multiple_entries_matching() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    let builder = RawEntryBuilder { map: &map};

    let hash = {
        let mut hasher = TestHasher.build_hasher();
        "key1".hash(&mut hasher);
        hasher.finish()
    };

    let result = builder.from_hash(hash, |k| k == &"key1" || k == &"key2");
    assert_eq!(result, Some((&"key1", &"value1")));
}

#[test]
fn test_from_hash_with_empty_map() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let map = IndexMap::new();
    let builder = RawEntryBuilder { map: &map };

    let hash = 0;

    let result = builder.from_hash(hash, |k| false);
    assert_eq!(result, None);
}

