// Answer 0

#[test]
fn test_is_empty_on_zero_capacity() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_on_initialized_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    assert!(map.is_empty());
    
    // Simulate adding elements to check non-empty case
    map.core.entries.push((1, 1));
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.core.entries.push((1, 1));
    assert!(!map.is_empty());
    
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_on_split_off() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.core.entries.push((1, 1));
    assert!(!map.is_empty());

    let split_map = map.split_off(1);
    // Check the original map still has elements
    assert!(!map.is_empty());
    // Check the split map has no elements, since we split at 1
    assert!(split_map.is_empty());
}

