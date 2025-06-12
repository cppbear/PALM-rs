// Answer 0

#[test]
fn test_shrink_to_fit_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.shrink_to_fit();
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_shrink_to_fit_non_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    assert!(map.len() > 0);
    map.shrink_to_fit();
    assert_eq!(map.capacity(), map.len());
}

#[test]
fn test_shrink_to_fit_multiple_insertions() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(4, RandomState::new());
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    assert!(map.len() == 10);
    map.shrink_to_fit();
    assert_eq!(map.capacity(), map.len());
}

#[test]
fn test_shrink_to_fit_after_clear() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 100);
    map.clear();
    assert!(map.is_empty());
    map.shrink_to_fit();
    assert_eq!(map.capacity(), 0);
}

