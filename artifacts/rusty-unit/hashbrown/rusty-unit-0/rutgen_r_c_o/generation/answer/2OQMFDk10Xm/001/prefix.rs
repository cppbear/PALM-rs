// Answer 0

#[test]
fn test_drain_non_empty_map() {
    let mut a = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    a.insert(1, "a");
    a.insert(2, "b");
    let _drain = a.drain();
}

#[test]
fn test_drain_empty_map() {
    let mut a = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    let _drain = a.drain();
    assert!(a.is_empty());
}

#[test]
fn test_drain_partial_iteration() {
    let mut a = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    a.insert(1, "a");
    a.insert(2, "b");
    let mut drain_iter = a.drain();
    let _ = drain_iter.next();
    assert!(a.is_empty());
}

#[test]
fn test_drain_with_capacity_preserved() {
    let mut a = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    a.insert(1, "a");
    a.insert(2, "b");
    let capacity_before_drain = a.capacity();
    let _drain = a.drain();
    assert_eq!(a.capacity(), capacity_before_drain);
}

#[test]
fn test_drain_without_consuming_iterator() {
    let mut a = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    a.insert(1, "a");
    a.insert(2, "b");
    {
        let _drain = a.drain();
    }
    assert!(a.is_empty());
}

