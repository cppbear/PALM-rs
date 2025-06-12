// Answer 0

#[test]
fn test_clear_when_map_is_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_with_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    assert!(!map.is_empty());
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_with_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    assert_eq!(map.len(), 3);
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_clear_with_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.clear();
    assert_eq!(map.capacity(), 5);
    assert!(map.is_empty());
}

#[test]
fn test_clear_after_resizing() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.reserve(5);
    map.clear();
    assert_eq!(map.capacity(), 3);
    assert!(map.is_empty());
}

#[test]
fn test_clear_on_full_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    assert_eq!(map.len(), 10);
    map.clear();
    assert!(map.is_empty());
}

