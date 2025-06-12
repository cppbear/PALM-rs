// Answer 0

#[test]
fn test_clear_on_empty_map() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let capacity_before_clear = map.capacity();
    
    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_on_non_empty_map() {
    let mut map = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.insert(2, "b");
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_on_large_map() {
    let mut map = HashMap::with_capacity_and_hasher_in(1000, DefaultHashBuilder::new(), Global);
    for i in 0..1000 {
        map.insert(i, "value");
    }
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

