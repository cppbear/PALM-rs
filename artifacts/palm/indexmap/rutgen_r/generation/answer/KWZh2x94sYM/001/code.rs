// Answer 0

#[derive(Debug)]
struct MockHasher;

struct IndexMapCore;

impl IndexMapCore {
    pub const fn new() -> Self {
        IndexMapCore
    }
}

struct IndexMap<S> {
    core: IndexMapCore,
    hash_builder: S,
}

impl<S> IndexMap<S> {
    pub const fn with_hasher(hash_builder: S) -> Self {
        IndexMap {
            core: IndexMapCore::new(),
            hash_builder,
        }
    }
}

#[test]
fn test_with_hasher() {
    let hasher = MockHasher;
    let map = IndexMap::with_hasher(hasher);
    assert_eq!(std::mem::size_of_val(&map.core), std::mem::size_of::<IndexMapCore>());
}

#[test]
fn test_with_hasher_boundary_condition() {
    let hasher = MockHasher;
    let map = IndexMap::with_hasher(hasher);
    assert_eq!(map.core.new(), IndexMapCore::new());
}

