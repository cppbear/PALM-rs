// Answer 0

#[test]
fn test_clear_on_non_empty_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "value1");
    map.insert(2, "value2");
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_on_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_on_large_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::with_capacity_and_hasher_in(1000, DefaultHashBuilder, Global);
    for i in 0..1000 {
        map.insert(i, "value");
    }
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

