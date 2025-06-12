// Answer 0

#[test]
fn test_drain_basic_functionality() {
    let mut map = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.insert(2, "b");
    let capacity_before_drain = map.capacity();

    let mut drained = map.drain();
    let (k, v) = drained.next().unwrap();
    assert!(k == 1 || k == 2);
    assert!(v == "a" || v == "b");

    assert!(map.is_empty() && map.len() == 0);
    assert_eq!(map.capacity(), capacity_before_drain);
}

#[test]
fn test_drain_partial_consumption() {
    let mut map = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.insert(2, "b");

    {
        let drained = map.drain();
        let _ = drained.take(1).collect::<Vec<_>>();
    }

    assert!(map.is_empty());
}

#[test]
fn test_drain_complete_consumption() {
    let mut map = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.insert(2, "b");

    let drained: Vec<_> = map.drain().collect();

    assert!(map.is_empty());
    assert_eq!(drained.len(), 2);
}

#[test]
fn test_drain_empty_map() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    let drained: Vec<_> = map.drain().collect();
    assert!(drained.is_empty());
}

