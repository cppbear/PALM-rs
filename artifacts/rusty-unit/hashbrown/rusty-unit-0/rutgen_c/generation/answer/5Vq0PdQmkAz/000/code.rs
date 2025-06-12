// Answer 0

#[test]
fn test_drain_non_empty_set() {
    let mut set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert!(!set.is_empty());
    
    let drained_elements: Vec<_> = set.drain().iter.collect(); // Change for correct method
    assert_eq!(drained_elements.len(), 3);
    assert!(set.is_empty());
}

#[test]
fn test_drain_empty_set() {
    let mut set: HashSet<i32> = HashSet::new();
    assert!(set.is_empty());
    
    let drained_elements: Vec<_> = set.drain().iter.collect(); // Change for correct method
    assert!(drained_elements.is_empty());
    assert!(set.is_empty());
}

