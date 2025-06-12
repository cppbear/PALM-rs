// Answer 0

#[test]
fn test_clear_empty_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_non_empty_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_map_with_large_capacity() {
    use hashbrown::HashMap;

    let mut map = HashMap::with_capacity(100);
    for i in 0..50 {
        map.insert(i, i);
    }
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

