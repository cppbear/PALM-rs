// Answer 0

#[test]
fn test_into_values_empty() {
    let map: HashMap<&str, i32> = HashMap::with_hasher_in(DefaultHashBuilder::new(), Global);
    let values: Vec<i32> = map.into_values().collect();
    assert!(values.is_empty());
}

#[test]
fn test_into_values_single_entry() {
    let mut map = HashMap::with_hasher_in(DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    let mut values: Vec<i32> = map.into_values().collect();
    values.sort_unstable();
    assert_eq!(values, [1]);
}

#[test]
fn test_into_values_multiple_entries() {
    let mut map = HashMap::with_hasher_in(DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let mut values: Vec<i32> = map.into_values().collect();
    values.sort_unstable();
    assert_eq!(values, [1, 2, 3]);
}

#[test]
fn test_into_values_ordering() {
    let mut map = HashMap::with_hasher_in(DefaultHashBuilder::new(), Global);
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);
    
    let mut values: Vec<i32> = map.into_values().collect();
    values.sort_unstable();
    assert_eq!(values, [10, 20, 30]);
}

#[should_panic]
fn test_into_values_map_used_after() {
    let mut map = HashMap::with_hasher_in(DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    let _values: IntoValues<&str, i32> = map.into_values();
    // The map should not be used after this point, uncommenting the next line should trigger a panic.
    // let _len = map.len();
}

