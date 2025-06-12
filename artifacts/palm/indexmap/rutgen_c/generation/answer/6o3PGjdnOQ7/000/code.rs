// Answer 0

#[test]
fn test_drain_full_range() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(5, std::collections::hash_map::RandomState::new());
    
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());

    let drained: super::Drain<i32> = index_set.drain(..);
    let drained_vec: Vec<i32> = drained.collect();

    assert_eq!(drained_vec, vec![1, 2, 3]);
    assert!(index_set.is_empty());
}

#[test]
#[should_panic(expected = "panics if starting point is greater than the end point")]
fn test_drain_invalid_range() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(3, std::collections::hash_map::RandomState::new());
    
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    index_set.drain(1..0); // This should panic
}

#[test]
#[should_panic(expected = "panics if end point is greater than the length of the set")]
fn test_drain_out_of_bounds() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(3, std::collections::hash_map::RandomState::new());
    
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    index_set.drain(0..3); // This should panic
}

#[test]
fn test_drain_partial_range() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(5, std::collections::hash_map::RandomState::new());
    
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());
    index_set.map.insert(4, ());
    
    let drained: super::Drain<i32> = index_set.drain(1..3);
    let drained_vec: Vec<i32> = drained.collect();

    assert_eq!(drained_vec, vec![2, 3]);
    assert_eq!(index_set.map.len(), 2);
    assert!(index_set.map.contains_key(&1));
    assert!(index_set.map.contains_key(&4));
}

