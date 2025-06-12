// Answer 0

#[test]
fn test_index_occupied() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut entries = indexmap::IndexMap::new();
    entries.insert("key1", "value1");
    let index = {
        let entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
            entries: &mut entries.entries,
            index: entries.get_index_of("key1").unwrap(),
            hash_builder: PhantomData,
        });
        entry.index()
    };
    assert_eq!(index, 0);
}

#[test]
fn test_index_vacant() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let map: indexmap::IndexMap<&str, &str, TestHasher> = indexmap::IndexMap::new();
    let entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&map),
        hash_builder: PhantomData,
    });
    
    let index = entry.index();
    assert_eq!(index, 0);
}

