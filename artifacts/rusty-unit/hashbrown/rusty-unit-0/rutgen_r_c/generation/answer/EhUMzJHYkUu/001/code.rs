// Answer 0

#[test]
fn test_is_empty_when_empty() {
    let map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_after_insert() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.clear();
    assert!(map.is_empty());
}

