// Answer 0

#[test]
fn test_drain_valid_range() {
    let mut set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.core.entries.push(Bucket { hash: 0, key: 1, value: () });
    set.map.core.entries.push(Bucket { hash: 0, key: 2, value: () });
    set.map.core.entries.push(Bucket { hash: 0, key: 3, value: () });
    
    let _drain = set.drain(0..2);
}

#[test]
fn test_drain_entire_set() {
    let mut set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    set.map.core.entries.push(Bucket { hash: 0, key: 1, value: () });
    set.map.core.entries.push(Bucket { hash: 0, key: 2, value: () });
    
    let _drain = set.drain(..);
}

#[test]
fn test_drain_empty_set() {
    let mut set = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    
    let _drain = set.drain(..);
}

#[test]
#[should_panic]
fn test_drain_invalid_start_greater_than_end() {
    let mut set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    set.map.core.entries.push(Bucket { hash: 0, key: 1, value: () });
    set.map.core.entries.push(Bucket { hash: 0, key: 2, value: () });
    
    let _drain = set.drain(2..1);
}

#[test]
#[should_panic]
fn test_drain_invalid_end_greater_than_length() {
    let mut set = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set.map.core.entries.push(Bucket { hash: 0, key: 1, value: () });
    
    let _drain = set.drain(0..3);
}

#[test]
fn test_drain_single_element() {
    let mut set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    set.map.core.entries.push(Bucket { hash: 0, key: 1, value: () });
    
    let _drain = set.drain(0..1);
}

