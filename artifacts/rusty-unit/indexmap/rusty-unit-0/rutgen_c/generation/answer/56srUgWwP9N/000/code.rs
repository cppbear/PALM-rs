// Answer 0

#[test]
fn test_from_hash_full_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::Hasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            // Return a custom hasher, this example is illustrative
        }
    }

    let mut map: IndexMap<u64, &str, TestHasher> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    let builder = RawEntryBuilder { map: &map };

    let result = builder.from_hash_full(1, |&key| key == 1);
    assert!(result.is_some());
    let (index, key, value) = result.unwrap();
    assert_eq!(index, 0);
    assert_eq!(*key, 1);
    assert_eq!(*value, "one");
}

#[test]
fn test_from_hash_full_not_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::Hasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            // Return a custom hasher, this example is illustrative
        }
    }

    let mut map: IndexMap<u64, &str, TestHasher> = IndexMap::new();
    map.insert(1, "one");

    let builder = RawEntryBuilder { map: &map };

    let result = builder.from_hash_full(2, |&key| key == 2);
    assert!(result.is_none());
}

