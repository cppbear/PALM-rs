// Answer 0


#[test]
fn test_from_hash_full_with_valid_hash() {
    struct TestHashMap {
        data: IndexMap<u32, String, std::collections::hash_map::RandomState>,
    }

    impl TestHashMap {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, "One".to_string());
            map.insert(2, "Two".to_string());
            map.insert(3, "Three".to_string());
            TestHashMap { data: map }
        }
    }

    let hash_map = TestHashMap::new();
    let builder = RawEntryBuilder { map: &hash_map.data };

    assert_eq!(
        builder.from_hash_full(2.hash(), |k| *k == 2),
        Some((1, &2, &"Two".to_string()))
    );
}

#[test]
fn test_from_hash_full_with_no_match() {
    struct TestHashMap {
        data: IndexMap<u32, String, std::collections::hash_map::RandomState>,
    }

    impl TestHashMap {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, "One".to_string());
            map.insert(2, "Two".to_string());
            TestHashMap { data: map }
        }
    }

    let hash_map = TestHashMap::new();
    let builder = RawEntryBuilder { map: &hash_map.data };

    assert_eq!(
        builder.from_hash_full(1.hash(), |k| *k == 3),
        None
    );
}

#[test]
fn test_from_hash_full_with_empty_map() {
    struct TestHashMap {
        data: IndexMap<u32, String, std::collections::hash_map::RandomState>,
    }

    impl TestHashMap {
        fn new() -> Self {
            TestHashMap {
                data: IndexMap::new(),
            }
        }
    }

    let hash_map = TestHashMap::new();
    let builder = RawEntryBuilder { map: &hash_map.data };

    assert_eq!(
        builder.from_hash_full(1.hash(), |k| *k == 1),
        None
    );
}


