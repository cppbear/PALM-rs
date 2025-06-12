// Answer 0

#[test]
fn test_iter_empty_hashmap() {
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    assert!(vec.is_empty());
}

#[test]
fn test_iter_single_entry() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0], ("a", 1));
}

#[test]
fn test_iter_multiple_entries() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    assert_eq!(vec.len(), 3);
    vec.sort_unstable();
    assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
}

#[test]
fn test_iter_large_hashmap() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1000, DefaultHashBuilder::new(), Global);
    for i in 0..1000 {
        map.insert(i, i * 2);
    }
    let mut vec: Vec<(i32, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    assert_eq!(vec.len(), 1000);
    vec.sort_unstable();
    for i in 0..1000 {
        assert_eq!(vec[i], (i, i * 2));
    }
}

