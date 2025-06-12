// Answer 0

#[test]
fn test_shift_remove_full_existing_key() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, MockHasher> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    let result = map.shift_remove_full(&2);
    assert_eq!(result, Some((1, 2, "two".to_string())));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&2), None);
}

#[test]
fn test_shift_remove_full_non_existing_key() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, MockHasher> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let result = map.shift_remove_full(&3);
    assert_eq!(result, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_shift_remove_full_single_element() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, MockHasher> = IndexMap::new();
    map.insert(1, "one".to_string());

    let result = map.shift_remove_full(&1);
    assert_eq!(result, Some((0, 1, "one".to_string())));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_shift_remove_full_empty_map() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, MockHasher> = IndexMap::new();

    let result = map.shift_remove_full(&1);
    assert_eq!(result, None);
    assert_eq!(map.len(), 0);
}

