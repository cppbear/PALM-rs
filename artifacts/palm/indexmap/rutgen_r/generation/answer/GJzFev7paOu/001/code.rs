// Answer 0

#[test]
fn test_drain_valid_range() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let drained: Vec<_> = map.drain(1..3).collect();
    assert_eq!(drained, vec![(2, "b"), (3, "c")]);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&"a"));
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_drain_panic_start_greater_than_end() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    
    let _ = map.drain(2..1);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_drain_panic_end_greater_than_map_length() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");

    let _ = map.drain(0..2);
}

#[test]
fn test_drain_entire_map() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");

    let drained: Vec<_> = map.drain(..).collect();
    assert_eq!(drained, vec![(1, "a"), (2, "b")]);
    assert!(map.is_empty());
}

