// Answer 0

#[test]
fn test_index_from_hash_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::FxHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut map = IndexMap::new(TestHasher);
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let builder = RawEntryBuilder { map: &map };
    let hash = "key1".hash() as u64;

    let result = builder.index_from_hash(hash, |key| key == "key1");
    assert_eq!(result, Some(0));
}

#[test]
fn test_index_from_hash_not_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::FxHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut map = IndexMap::new(TestHasher);
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let builder = RawEntryBuilder { map: &map };
    let hash = "key3".hash() as u64;

    let result = builder.index_from_hash(hash, |key| key == "key3");
    assert_eq!(result, None);
}

#[test]
fn test_index_from_hash_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::FxHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let map = IndexMap::new(TestHasher);
    let builder = RawEntryBuilder { map: &map };
    let hash = "key1".hash() as u64;

    let result = builder.index_from_hash(hash, |key| key == "key1");
    assert_eq!(result, None);
}

