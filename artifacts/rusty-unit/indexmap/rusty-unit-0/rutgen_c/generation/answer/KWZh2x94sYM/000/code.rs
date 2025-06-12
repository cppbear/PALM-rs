// Answer 0

#[test]
fn test_index_map_with_hasher() {
    struct MockHasher;
    
    impl BuildHasher for MockHasher {
        type Hasher = Self;

        fn build_hasher(&self) -> Self::Hasher {
            MockHasher
        }
    }

    let hasher = MockHasher;
    let map: IndexMap<i32, &str, MockHasher> = IndexMap::with_hasher(hasher);

    assert!(map.len() == 0);
    assert!(map.is_empty());
}

