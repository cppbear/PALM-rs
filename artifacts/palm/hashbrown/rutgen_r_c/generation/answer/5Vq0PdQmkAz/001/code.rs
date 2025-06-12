// Answer 0

#[test]
fn test_drain_non_empty_set() {
    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert!(!set.is_empty());
    
    let mut drained_elements = Vec::new();
    for i in set.drain() {
        drained_elements.push(i);
    }

    assert!(set.is_empty());
    assert_eq!(drained_elements.len(), 3);
}

#[test]
fn test_drain_empty_set() {
    let mut set: HashSet<_> = HashSet::new();
    assert!(set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert!(set.is_empty());
    assert!(drained_elements.is_empty());
}

#[test]
fn test_drain_large_set() {
    let mut set: HashSet<_> = (1..=1000).collect();
    assert!(!set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert!(set.is_empty());
    assert_eq!(drained_elements.len(), 1000);
}

#[test]
fn test_drain_with_duplicate_elements() {
    let mut set: HashSet<_> = vec![1, 1, 2, 2, 3, 3].into_iter().collect();
    assert!(!set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert!(set.is_empty());
    assert_eq!(drained_elements.len(), 3);
}

