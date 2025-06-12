// Answer 0

#[test]
fn test_keys_with_empty_map() {
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let keys: Keys<'_, &str, i32> = map.keys();
    let mut vec: Vec<&&str> = Vec::new();

    for key in keys {
        vec.push(key);
    }

    assert!(vec.is_empty());
}

#[test]
fn test_keys_with_single_element() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    let keys: Keys<'_, &str, i32> = map.keys();
    let mut vec: Vec<&&str> = Vec::new();

    for key in keys {
        vec.push(key);
    }

    assert_eq!(vec, [&"a"]);
}

#[test]
fn test_keys_with_multiple_elements() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let keys: Keys<'_, &str, i32> = map.keys();
    let mut vec: Vec<&&str> = Vec::new();

    for key in keys {
        vec.push(key);
    }

    vec.sort_unstable();
    assert_eq!(vec, [&"a", &"b", &"c"]);
}

#[test]
fn test_keys_boundary_conditions() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    
    let keys: Keys<'_, &str, i32> = map.keys();
    let mut vec: Vec<&&str> = Vec::new();

    for key in keys {
        vec.push(key);
    }

    assert_eq!(vec.len(), 2);
    assert!(vec.contains(&&"a"));
    assert!(vec.contains(&&"b"));
}

