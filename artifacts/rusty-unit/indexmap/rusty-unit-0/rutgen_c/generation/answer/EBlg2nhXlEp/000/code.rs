// Answer 0

#[test]
fn test_from_key_hashed_nocheck_found() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::hash::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key_hashed_nocheck(1, &1);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (&1, &10));
}

#[test]
fn test_from_key_hashed_nocheck_not_found() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::hash::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key_hashed_nocheck(2, &3);
    assert!(result.is_none());
}

