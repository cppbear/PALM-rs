// Answer 0

#[test]
fn test_from_key_existing_key() {
    struct FakeHasher;

    impl BuildHasher for FakeHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, FakeHasher> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);
    
    let builder = RawEntryBuilder { map: &map };
    let key = "a";
    
    assert_eq!(builder.from_key(&key), Some((&"a", &100)));
}

#[test]
fn test_from_key_non_existing_key() {
    struct FakeHasher;

    impl BuildHasher for FakeHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, u32, FakeHasher> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);
    
    let builder = RawEntryBuilder { map: &map };
    let key = "c";
    
    assert_eq!(builder.from_key(&key), None);
}

#[test]
fn test_from_key_empty_map() {
    struct FakeHasher;

    impl BuildHasher for FakeHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let map: HashMap<&str, u32, FakeHasher> = HashMap::new();
    
    let builder = RawEntryBuilder { map: &map };
    let key = "a";
    
    assert_eq!(builder.from_key(&key), None);
}

