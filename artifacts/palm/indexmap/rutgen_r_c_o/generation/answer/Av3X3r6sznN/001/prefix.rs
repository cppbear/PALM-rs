// Answer 0

#[test]
fn test_hasher_with_default_hasher() {
    use std::collections::hash_map::RandomState;
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let hasher = map.hasher();
}

#[test]
fn test_hasher_with_non_empty_map() {
    use std::collections::hash_map::RandomState;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.try_reserve(3).unwrap();
    map.clear();
    let hasher = map.hasher();
}

#[test]
fn test_hasher_with_large_capacity() {
    use std::collections::hash_map::RandomState;
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1000000, RandomState::new());
    let hasher = map.hasher();
}

#[test]
fn test_hasher_when_empty() {
    use std::collections::hash_map::RandomState;
    let map: IndexMap<char, char, RandomState> = IndexMap::with_hasher(RandomState::new());
    let hasher = map.hasher();
}

#[test]
#[should_panic]
fn test_hasher_with_invalid_usage() {
    use std::collections::hash_map::RandomState;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.drain(0..1); // This should not panic, but just to test invalid range drain
    let hasher = map.hasher();
}

