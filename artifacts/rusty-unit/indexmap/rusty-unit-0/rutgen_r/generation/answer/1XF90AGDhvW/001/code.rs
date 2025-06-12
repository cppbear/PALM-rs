// Answer 0

#[test]
fn test_contains_key_existing_key() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Key {
        value: usize,
    }

    let mut map = IndexMap::new();
    map.insert(Key { value: 1 }, "value1");
    map.insert(Key { value: 2 }, "value2");

    assert!(map.contains_key(&Key { value: 1 }));
    assert!(map.contains_key(&Key { value: 2 }));
}

#[test]
fn test_contains_key_non_existing_key() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Key {
        value: usize,
    }

    let map: IndexMap<Key, &str> = IndexMap::new();
    map.insert(Key { value: 1 }, "value1");

    assert!(!map.contains_key(&Key { value: 3 }));
}

#[test]
fn test_contains_key_empty_map() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Key {
        value: usize,
    }

    let map: IndexMap<Key, &str> = IndexMap::new();

    assert!(!map.contains_key(&Key { value: 1 }));
}

#[test]
fn test_contains_key_with_different_struct() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Key {
        value: usize,
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct OtherKey {
        value: usize,
    }

    let mut map = IndexMap::new();
    map.insert(Key { value: 1 }, "value1");

    assert!(!map.contains_key(&OtherKey { value: 1 }));
}

#[test]
#[should_panic]
fn test_contains_key_non_hashable_key() {
    use indexmap::IndexMap;

    struct NonHashableKey;

    let map: IndexMap<NonHashableKey, &str> = IndexMap::new();
    map.contains_key(&NonHashableKey);
}

