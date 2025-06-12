// Answer 0

#[test]
fn test_into_keys_empty() {
    let map: HashMap<&str, i32> = HashMap::new();
    let keys: IntoKeys<_, _, _> = map.into_keys();
    let vec: Vec<&str> = keys.collect();
    assert!(vec.is_empty());
}

#[test]
fn test_into_keys_single_element() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("single", 42);
    let mut keys: IntoKeys<_, _, _> = map.into_keys();
    let vec: Vec<&str> = keys.collect();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0], "single");
}

#[test]
fn test_into_keys_multiple_elements() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let mut keys: IntoKeys<_, _, _> = map.into_keys();
    let mut vec: Vec<&str> = keys.collect();
    vec.sort_unstable();
    assert_eq!(vec, ["a", "b", "c"]);
}

#[test]
fn test_into_keys_after_consumption() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    let keys: IntoKeys<_, _, _> = map.into_keys();
    let _vec: Vec<&str> = keys.collect();
    // Trying to access the map after calling into_keys should panic
    // However, as per the instruction, we do not add this checking.
    // This is to showcase the intended use only.
}

#[test]
fn test_into_keys_with_capacity() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), Global);
    map.insert("x", 100);
    map.insert("y", 200);
    let mut keys: IntoKeys<_, _, _> = map.into_keys();
    let mut vec: Vec<&str> = keys.collect();
    vec.sort_unstable();
    assert_eq!(vec, ["x", "y"]);
}

