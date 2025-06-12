// Answer 0

#[test]
fn test_drain_full_range() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    let drained: Vec<i32> = set.drain(..).collect();
    assert_eq!(drained, vec![1, 2, 3]);
    assert!(set.is_empty());
}

#[test]
fn test_drain_middle_range() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    
    let drained: Vec<i32> = set.drain(1..4).collect();
    assert_eq!(drained, vec![2, 3, 4]);
    assert_eq!(set.len(), 2);
    assert!(set.contains(&1));
    assert!(set.contains(&5));
}

#[test]
fn test_drain_empty_set() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    
    let drained: Vec<i32> = set.drain(..).collect();
    assert_eq!(drained, vec![]);
    assert!(set.is_empty());
}

#[test]
#[should_panic(expected = "start index must be less than or equal to end index")]
fn test_drain_invalid_range_start_greater_than_end() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.insert(1);
    set.insert(2);
    
    let _drained: Vec<i32> = set.drain(1..0).collect();
}

#[test]
#[should_panic(expected = "end index out of bounds")]
fn test_drain_invalid_range_end_greater_than_length() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.insert(1);
    
    let _drained: Vec<i32> = set.drain(0..2).collect();
}

#[test]
fn test_drain_single_element() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.insert(1);
    
    let drained: Vec<i32> = set.drain(0..1).collect();
    assert_eq!(drained, vec![1]);
    assert!(set.is_empty());
}

