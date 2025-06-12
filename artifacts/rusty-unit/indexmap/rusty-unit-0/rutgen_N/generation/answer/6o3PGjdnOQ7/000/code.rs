// Answer 0

#[test]
fn test_drain_full_range() {
    let mut set = indexmap::IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    let drained: Vec<_> = set.drain(..).collect();
    assert_eq!(drained, vec![1, 2, 3]);
    assert!(set.is_empty());
}

#[test]
fn test_drain_partial_range() {
    let mut set = indexmap::IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    let drained: Vec<_> = set.drain(1..3).collect();
    assert_eq!(drained, vec![2, 3]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&1));
}

#[test]
fn test_drain_empty_set() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    
    let drained: Vec<_> = set.drain(..).collect();
    assert!(drained.is_empty());
    assert!(set.is_empty());
}

#[test]
#[should_panic]
fn test_drain_invalid_range_start_greater_end() {
    let mut set = indexmap::IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    let _ = set.drain(2..1);
}

#[test]
#[should_panic]
fn test_drain_invalid_range_end_greater_length() {
    let mut set = indexmap::IndexSet::new();
    set.insert(1);
    
    let _ = set.drain(..2);
}

