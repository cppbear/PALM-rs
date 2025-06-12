// Answer 0

#[test]
fn test_drain_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let drain = map.drain(..);
    assert!(drain.as_slice().is_empty());
}

#[test]
fn test_drain_full() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let drain = map.drain(..);
    assert_eq!(drain.as_slice().len(), 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_drain_out_of_bounds_start() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let _ = map.drain(2..5); // Start is valid, but end is out of bounds.
}

#[test]
#[should_panic(expected = "invalid range")]
fn test_drain_invalid_range() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    let _ = map.drain(2..1); // Start is greater than end.
}

#[test]
fn test_drain_partial() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    let drain = map.drain(1..3); // Drain items at index 1 and 2.
    assert_eq!(drain.as_slice().len(), 2);
    assert!(map.len() == 3); // Remaining items should be 3.
}

