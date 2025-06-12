// Answer 0

#[test]
fn test_drain_empty_range() {
    let mut index_map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    let drain = index_map.drain(..);
    assert_eq!(drain.as_slice().len(), 0);
}

#[test]
fn test_drain_full_range() {
    let mut index_map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    
    let drain = index_map.drain(..);
    let drained: Vec<_> = drain.as_slice().to_vec();
    assert_eq!(drained.len(), 3);
    assert_eq!(drained, vec![(1, 10), (2, 20), (3, 30)]);
    assert!(index_map.is_empty());
}

#[test]
#[should_panic]
fn test_drain_invalid_start() {
    let mut index_map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    index_map.insert(1, 10);
    let _ = index_map.drain(1..0); // start > end
}

#[test]
#[should_panic]
fn test_drain_invalid_end() {
    let mut index_map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    index_map.insert(1, 10);
    let _ = index_map.drain(0..2); // end index out of bounds
}

#[test]
fn test_drain_partial_range() {
    let mut index_map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    
    let drain = index_map.drain(1..3);
    let drained: Vec<_> = drain.as_slice().to_vec();
    assert_eq!(drained.len(), 2);
    assert_eq!(drained, vec![(2, 20), (3, 30)]);
    assert_eq!(index_map.len(), 1);
    assert!(index_map.contains_key(&1));
}

