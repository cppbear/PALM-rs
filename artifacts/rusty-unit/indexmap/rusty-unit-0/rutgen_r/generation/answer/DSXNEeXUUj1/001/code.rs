// Answer 0

#[test]
fn test_new_index_map_core() {
    struct Indices;

    impl Indices {
        const fn new() -> Self {
            Indices
        }
    }

    struct IndexMapCore {
        indices: Indices,
        entries: Vec<u8>,
    }

    impl IndexMapCore {
        const fn new() -> Self {
            IndexMapCore {
                indices: Indices::new(),
                entries: Vec::new(),
            }
        }
    }

    let index_map_core = IndexMapCore::new();
    assert_eq!(index_map_core.entries.len(), 0);
}

