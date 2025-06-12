// Answer 0

#[test]
fn test_index_map_core_new() {
    struct IndexMapCore {
        indices: Indices,
        entries: Vec<u8>, // assuming some type for entries
    }

    struct Indices;

    impl Indices {
        pub(crate) const fn new() -> Self {
            Indices {}
        }
    }

    let map_core = IndexMapCore::new();
    assert!(map_core.entries.is_empty());
}

