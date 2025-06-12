// Answer 0

#[test]
fn test_with_hasher() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    struct IndexSet<S> {
        map: IndexMap<u32, ()>,
    }

    impl<S: Default> IndexSet<S> {
        pub const fn with_hasher(hash_builder: S) -> Self {
            IndexSet {
                map: IndexMap::with_hasher(hash_builder),
            }
        }
    }

    let hash_builder = RandomState::new();
    let set = IndexSet::with_hasher(hash_builder);
    assert!(set.map.is_empty());
}

#[test]
fn test_with_default_hasher() {
    use indexmap::IndexMap;

    struct IndexSet<S> {
        map: IndexMap<u32, ()>,
    }

    impl<S: Default> IndexSet<S> {
        pub const fn with_hasher(hash_builder: S) -> Self {
            IndexSet {
                map: IndexMap::with_hasher(hash_builder),
            }
        }
    }

    let set = IndexSet::with_hasher(Default::default());
    assert!(set.map.is_empty());
}

