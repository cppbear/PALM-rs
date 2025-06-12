// Answer 0

#[test]
fn test_raw_entry_mut_index_vacant() {
    struct MockHasher;
    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct MockMap {
        indices: Vec<usize>,
    }

    let mock_map = MockMap { indices: vec![] };
    let hash_builder = MockHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mock_map),
        hash_builder: &hash_builder,
    });

    assert_eq!(vacant_entry.index(), 0);
}

