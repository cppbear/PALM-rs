// Answer 0

#[test]
fn test_is_empty_initially_empty() {
    let map: HashMap<i32, &str> = HashMap::new();
    let _ = map.is_empty();
}

#[test]
fn test_is_empty_after_insertion() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "a");
    let _ = map.is_empty();
}

#[test]
fn test_is_empty_after_clear() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "a");
    map.clear();
    let _ = map.is_empty();
}

#[test]
fn test_is_empty_with_capacity() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    let _ = map.is_empty();
}

#[test]
fn test_is_empty_with_multiple_insertions() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    let _ = map.is_empty();
}

