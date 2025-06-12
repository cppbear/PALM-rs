// Answer 0

#[test]
fn test_get_full_mut2_nonexistent_key() {
    struct SimpleEquivalent;

    impl Equivalent<i32> for SimpleEquivalent {
        fn equivalent(&self, _: &i32) -> bool {
            false
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    let key: &SimpleEquivalent = &SimpleEquivalent;

    // This key does not exist in the map, so we expect None.
    let result = map.get_full_mut2(key);
    assert_eq!(result, None);
}

#[test]
fn test_get_full_mut2_empty_map() {
    struct SimpleEquivalent;

    impl Equivalent<i32> for SimpleEquivalent {
        fn equivalent(&self, _: &i32) -> bool {
            false
        }
    }

    let mut map: IndexMap<i32, String, SimpleHasher> = IndexMap::new();
    let key: &SimpleEquivalent = &SimpleEquivalent;

    // This key does not exist in the empty map, expecting None.
    let result = map.get_full_mut2(key);
    assert_eq!(result, None);
}

struct SimpleHasher;

impl BuildHasher for SimpleHasher {
    type Hasher = std::collections::hash_map::RandomState;

    fn build_hasher(&self) -> Self::Hasher {
        std::collections::hash_map::RandomState::new()
    }
}

