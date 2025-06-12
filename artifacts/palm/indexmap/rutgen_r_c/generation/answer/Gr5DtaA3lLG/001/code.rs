// Answer 0

#[test]
fn test_into_keys_with_empty_map() {
    use std::collections::hash_map::RandomState;

    let map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let keys: IntoKeys<u32, String> = map.into_keys();
    assert_eq!(keys.iter.len(), 0);
}

#[test]
fn test_into_keys_with_single_entry() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, "one".to_string());
    let keys: IntoKeys<u32, String> = map.into_keys();
    assert_eq!(keys.iter.len(), 1);
    assert_eq!(keys.iter.next(), Some(1));
}

#[test]
fn test_into_keys_with_multiple_entries() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    let keys: IntoKeys<u32, String> = map.into_keys();

    let collected_keys: Vec<_> = keys.iter.collect();
    assert_eq!(collected_keys, vec![1, 2, 3]);
}

#[test]
fn test_into_keys_with_capacity_growth() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.reserve(3);
    map.insert(3, "three".to_string());
    let keys: IntoKeys<u32, String> = map.into_keys();

    let collected_keys: Vec<_> = keys.iter.collect();
    assert_eq!(collected_keys, vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_into_keys_on_invalid_index_access() {
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map.insert(1, "one".to_string());
    map.into_keys(); // ensuring the map is consumed
    map.get(&1); // this should panic since the map has been moved
}

