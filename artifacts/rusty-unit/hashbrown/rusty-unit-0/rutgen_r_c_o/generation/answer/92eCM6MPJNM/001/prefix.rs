// Answer 0

#[test]
fn test_into_keys_non_empty_map() {
    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let keys_iterator = map.into_keys();
}

#[test]
fn test_into_keys_single_element_map() {
    let mut map = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("only_key", 42);
    let keys_iterator = map.into_keys();
}

#[test]
fn test_into_keys_empty_map() {
    let map: HashMap<String, i32, DefaultHashBuilder, Global> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let keys_iterator = map.into_keys();
}

#[test]
fn test_into_keys_large_map() {
    let mut map = HashMap::with_capacity_and_hasher_in(1000, DefaultHashBuilder::new(), Global);
    for i in 0..1000 {
        map.insert(i.to_string(), i);
    }
    let keys_iterator = map.into_keys();
}

#[should_panic]
fn test_into_keys_after_consumption() {
    let mut map = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert("key1", 1);
    map.insert("key2", 2);
    let keys_iterator = map.into_keys();
    let _ = map.keys(); // This should trigger a panic as the map cannot be used after into_keys
}

