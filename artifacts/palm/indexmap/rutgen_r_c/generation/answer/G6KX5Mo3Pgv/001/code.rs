// Answer 0

#[test]
fn test_truncate_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.truncate(0);
    assert!(map.is_empty());
}

#[test]
fn test_truncate_exceeding_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.truncate(5);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_truncate_same_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.truncate(2);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_truncate_fewer_elements() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.truncate(1);
    assert_eq!(map.len(), 1);
    assert_eq!(map.iter().next().unwrap().0, &1);
}

#[test]
fn test_truncate_zero_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.truncate(0);
    assert_eq!(map.len(), 0);
}

