// Answer 0

#[test]
fn test_is_empty_initially_empty() {
    let map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), Global);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_after_insertion() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), Global);
    map.insert(1, "a");
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clearing() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), Global);
    map.insert(1, "a");
    map.clear();
    assert!(map.is_empty());
}

