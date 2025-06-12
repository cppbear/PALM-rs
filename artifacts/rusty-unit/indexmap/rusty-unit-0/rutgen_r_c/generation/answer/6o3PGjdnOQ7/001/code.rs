// Answer 0

#[test]
fn test_drain_full_range() {
    let mut index_set = IndexSet::<u32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    index_set.map.core.entries.vec.push(Bucket { hash: 0, key: 1, value: () });
    index_set.map.core.entries.vec.push(Bucket { hash: 1, key: 2, value: () });
    index_set.map.core.entries.vec.push(Bucket { hash: 2, key: 3, value: () });
    
    let drain_iter = index_set.drain(..);
    assert_eq!(drain_iter.as_slice().len(), 3);
    assert!(index_set.is_empty());
}

#[test]
#[should_panic(expected = "start index greater than end index")]
fn test_drain_invalid_range_start_greater() {
    let mut index_set = IndexSet::<u32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    index_set.map.core.entries.vec.push(Bucket { hash: 0, key: 1, value: () });
    index_set.map.core.entries.vec.push(Bucket { hash: 1, key: 2, value: () });
    
    index_set.drain(1..0);
}

#[test]
#[should_panic(expected = "end index greater than the length of the set")]
fn test_drain_end_index_out_of_bounds() {
    let mut index_set = IndexSet::<u32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    index_set.map.core.entries.vec.push(Bucket { hash: 0, key: 1, value: () });
    
    index_set.drain(0..2);
}

#[test]
fn test_drain_partial_range() {
    let mut index_set = IndexSet::<u32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    index_set.map.core.entries.vec.push(Bucket { hash: 0, key: 1, value: () });
    index_set.map.core.entries.vec.push(Bucket { hash: 1, key: 2, value: () });
    index_set.map.core.entries.vec.push(Bucket { hash: 2, key: 3, value: () });
    
    let drain_iter = index_set.drain(0..2);
    assert_eq!(drain_iter.as_slice().len(), 2);
    assert_eq!(index_set.map.core.entries.vec.len(), 1);
    assert_eq!(index_set.map.core.entries.vec[0].key, 3);
}

#[test]
fn test_drain_empty_set() {
    let mut index_set = IndexSet::<u32, RandomState>::with_capacity_and_hasher(0, RandomState::new());
    
    let drain_iter = index_set.drain(..);
    assert_eq!(drain_iter.as_slice().len(), 0);
    assert!(index_set.is_empty());
}

