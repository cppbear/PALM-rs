// Answer 0

#[test]
fn test_and_modify_on_occupied_entry() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, MockHasher> = HashMap::new();
    map.insert("key", 1);

    let entry = map.entry("key");
    let modified_entry = entry.and_modify(|value| {
        *value += 10;
    });

    if let Entry::Occupied(occupied_entry) = modified_entry {
        assert_eq!(*occupied_entry.get(), 11);
    } else {
        panic!("Expected an occupied entry");
    }
}

#[test]
fn test_and_modify_on_vacant_entry() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, MockHasher> = HashMap::new();
    let entry = map.entry("missing_key");
    
    let modified_entry = entry.and_modify(|value| {
        *value += 10;
    });

    if let Entry::Vacant(_) = modified_entry {
        assert!(map.get("missing_key").is_none());
    } else {
        panic!("Expected a vacant entry");
    }
}

#[test]
fn test_and_modify_with_functional_update() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, MockHasher> = HashMap::new();
    map.insert("key", 1);

    map.entry("key")
        .and_modify(|value| {
            *value += 5;
        })
        .or_insert(42);

    assert_eq!(map["key"], 6);
}

