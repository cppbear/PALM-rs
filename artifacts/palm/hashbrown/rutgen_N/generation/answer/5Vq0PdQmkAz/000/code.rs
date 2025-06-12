// Answer 0

#[test]
fn test_drain_non_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert!(!set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert!(set.is_empty());
    assert_eq!(drained_elements.len(), 3);
    assert!(drained_elements.contains(&1));
    assert!(drained_elements.contains(&2));
    assert!(drained_elements.contains(&3));
}

#[test]
fn test_drain_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    assert!(set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert!(set.is_empty());
    assert!(drained_elements.is_empty());
}

