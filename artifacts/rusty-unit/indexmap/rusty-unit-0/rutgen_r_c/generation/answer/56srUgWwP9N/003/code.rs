// Answer 0

#[test]
fn test_from_hash_full_valid_case() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestHasher(DefaultHasher);

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map = IndexMap::new(TestHasher(DefaultHasher::new()));
    let key = "test_key";
    let value = "test_value";
    map.insert(key.to_string(), value.to_string());

    let builder = RawEntryBuilder { map: &map };
    
    let hash = {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    };

    let result = builder.from_hash_full(hash, |k| k == key);
    assert!(result.is_some());
    let (i, k, v) = result.unwrap();
    assert_eq!(k, &key);
    assert_eq!(v, &value);
}

#[test]
fn test_from_hash_full_not_found_case() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestHasher(DefaultHasher);

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map = IndexMap::new(TestHasher(DefaultHasher::new()));
    let builder = RawEntryBuilder { map: &map };

    let hash = 12345; // Arbitrary hash that does not correspond to any key
    let result = builder.from_hash_full(hash, |_| false);
    assert!(result.is_none());
}

