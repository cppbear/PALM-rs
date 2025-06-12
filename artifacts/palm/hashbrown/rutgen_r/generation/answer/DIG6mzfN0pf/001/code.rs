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
fn test_clear_after_multiple_insertions() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert(i, i.to_string());
    }
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

#[test]
fn test_clear_when_reused() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(10, "ten");
    map.clear();
    let capacity_after_clear = map.capacity();

    map.insert(20, "twenty");
    assert_eq!(map.capacity(), capacity_after_clear);
    assert!(!map.is_empty());
    assert_eq!(map.get(&20), Some(&"twenty"));
}

#[test]
fn test_clear_capacity_after_insertions() {
    use hashbrown::HashMap;

    let mut map = HashMap::<i32, &str>::new();
    for i in 0..500 {
        map.insert(i, "value");
    }
    let capacity_before_clear = map.capacity();

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_clear);
}

