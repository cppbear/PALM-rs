// Answer 0

#[test]
fn test_iter_empty_map() {
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), Global);
    let mut iter = map.iter();
}

#[test]
fn test_iter_single_element() {
    let mut map = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), Global);
    map.insert("a", 1);
    let mut iter = map.iter();
}

#[test]
fn test_iter_multiple_elements() {
    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::default(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let mut iter = map.iter();
}

#[test]
fn test_iter_large_capacity() {
    let mut map = HashMap::with_capacity_and_hasher_in(1_000_000, DefaultHashBuilder::default(), Global);
    for i in 0..1_000_000 {
        map.insert(i.to_string(), i);
    }
    let mut iter = map.iter();
}

#[test]
fn test_iter_repeated_elements() {
    let mut map = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::default(), Global);
    map.insert("a", 1);
    map.insert("a", 2);  // Update value for the same key
    map.insert("b", 3);
    let mut iter = map.iter();
}

