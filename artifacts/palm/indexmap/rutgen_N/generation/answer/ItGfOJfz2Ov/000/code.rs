// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    use indexmap::IndexMap; // Assume this is how you bring in your IndexMap
    use std::collections::hash_map::RandomState; // Assume we use RandomState as our hasher

    let map: IndexMap<i32, i32> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    use indexmap::IndexMap; // Assume this is how you bring in your IndexMap
    use std::collections::hash_map::RandomState; // Assume we use RandomState as our hasher

    let initial_capacity = 10;
    let map: IndexMap<i32, i32> = IndexMap::with_capacity_and_hasher(initial_capacity, RandomState::new());
    assert_eq!(map.capacity(), initial_capacity);
}

#[test]
fn test_with_capacity_and_hasher_large_capacity() {
    use indexmap::IndexMap; // Assume this is how you bring in your IndexMap
    use std::collections::hash_map::RandomState; // Assume we use RandomState as our hasher

    let initial_capacity = usize::MAX;
    let map: IndexMap<i32, i32> = IndexMap::with_capacity_and_hasher(initial_capacity, RandomState::new());
    assert_eq!(map.capacity(), initial_capacity);
}

