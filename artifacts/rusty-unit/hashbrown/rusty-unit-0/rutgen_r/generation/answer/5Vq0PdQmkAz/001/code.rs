// Answer 0

#[test]
fn test_drain_with_non_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
    assert!(!set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert_eq!(drained_elements.len(), 3);
    assert!(set.is_empty());
}

#[test]
fn test_drain_with_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    assert!(set.is_empty());

    let drained_elements: Vec<i32> = set.drain().collect();
    assert!(drained_elements.is_empty());
    assert!(set.is_empty());
}

#[test]
fn test_drain_with_boundary_elements() {
    use hashbrown::HashSet;

    let mut set: HashSet<_> = (1..=1000).collect();
    assert!(!set.is_empty());

    let drained_elements: Vec<_> = set.drain().collect();
    assert_eq!(drained_elements.len(), 1000);
    assert!(set.is_empty());
}

