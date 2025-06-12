// Answer 0

#[test]
fn test_truncate_below_capacity() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    set.truncate(2);
    
    assert_eq!(set.len(), 2);
}

#[test]
fn test_truncate_exact_capacity() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    set.truncate(3);
    
    assert_eq!(set.len(), 3);
}

#[test]
fn test_truncate_above_capacity() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    set.truncate(10); // should not panic, and should have no effect
    
    assert_eq!(set.len(), 3);
}

#[test]
fn test_truncate_empty_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    set.truncate(0);
    
    assert_eq!(set.len(), 0);
}

#[test]
fn test_truncate_to_zero() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    set.truncate(0);
    
    assert_eq!(set.len(), 0);
}

