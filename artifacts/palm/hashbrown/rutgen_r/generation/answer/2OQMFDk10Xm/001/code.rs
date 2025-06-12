// Answer 0

#[test]
fn test_drain_empty_map() {
    use hashbrown::HashMap;

    let mut a = HashMap::new();
    let drain_iter = a.drain();
    let mut drained_pairs: Vec<_> = drain_iter.collect();

    assert_eq!(drained_pairs.len(), 0);
    assert!(a.is_empty());
}

#[test]
fn test_drain_map_with_elements() {
    use hashbrown::HashMap;

    let mut a = HashMap::new();
    a.insert(1, "a");
    a.insert(2, "b");
    let capacity_before_drain = a.capacity();

    let mut drained_pairs: Vec<_> = a.drain().collect();

    assert_eq!(drained_pairs.len(), 2);
    assert!(drained_pairs.contains(&(1, "a")));
    assert!(drained_pairs.contains(&(2, "b")));
    assert!(a.is_empty());
    assert_eq!(a.capacity(), capacity_before_drain);
}

#[test]
fn test_drain_partial_consumption() {
    use hashbrown::HashMap;

    let mut a = HashMap::new();
    a.insert(3, "c");
    a.insert(4, "d");
    let capacity_before_drain = a.capacity();

    let mut drain_iter = a.drain();

    // Take only one element from the drain iterator
    assert_eq!(drain_iter.next(), Some((3, "c")));
    
    // The map should still not be empty
    assert!(!a.is_empty());
    assert_eq!(a.len(), 1);

    // Consume rest of the iterator
    let remaining: Vec<_> = drain_iter.collect();
    assert_eq!(remaining, vec![(4, "d")]);
    assert!(a.is_empty());
    assert_eq!(a.capacity(), capacity_before_drain);
}

#[test]
#[should_panic]
fn test_drain_after_drop() {
    use hashbrown::HashMap;

    let mut a = HashMap::new();
    a.insert(5, "e");
    a.insert(6, "f");

    {
        let _drain_iter = a.drain();
        // Dropping the iterator here
    }

    // Map should be empty even after drain iterator is dropped
    assert!(a.is_empty());
}

