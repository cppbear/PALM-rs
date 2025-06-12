// Answer 0

#[test]
fn test_iter_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    assert!(vec.is_empty());
}

#[test]
fn test_iter_single_element() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    vec.sort_unstable();
    assert_eq!(vec, [("a", 1)]);
}

#[test]
fn test_iter_multiple_elements() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    // Sort to ensure order is arbitrary
    vec.sort_unstable();
    assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
}

#[test]
fn test_iter_with_duplicates() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("a", 2); // Overwrite the value for the key "a"
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    vec.sort_unstable();
    assert_eq!(vec, [("a", 2)]); // Only the last value should be retained
}

