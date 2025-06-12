// Answer 0

#[test]
fn test_drain_functionality() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    let capacity_before_drain = map.capacity();

    let drained: Vec<(i32, &str)> = map.drain().take(1).collect();
    assert_eq!(drained.len(), 1);
    assert!(drained[0].0 == 1 || drained[0].0 == 2);
    assert!(drained[0].1 == "a" || drained[0].1 == "b");

    assert!(map.is_empty() && map.len() == 0);
    assert_eq!(map.capacity(), capacity_before_drain);
}

#[test]
fn test_drain_iterator_dropped() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");

    {
        let _drain = map.drain();
    }

    assert!(map.is_empty());
}

