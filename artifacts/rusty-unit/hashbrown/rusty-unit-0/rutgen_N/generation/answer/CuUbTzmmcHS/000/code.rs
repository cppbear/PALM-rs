// Answer 0

#[test]
fn test_iter_with_string_keys_and_integer_values() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    assert_eq!(map.len(), 3);

    let mut vec: Vec<(&str, i32)> = Vec::new();
    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    vec.sort_unstable();
    assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_iter_with_float_keys_and_string_values() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1.1, "one point one");
    map.insert(2.2, "two point two");
    map.insert(3.3, "three point three");
    assert_eq!(map.len(), 3);

    let mut vec: Vec<(f64, &str)> = Vec::new();
    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    vec.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    assert_eq!(vec, [(1.1, "one point one"), (2.2, "two point two"), (3.3, "three point three")]);
    assert_eq!(map.len(), 3);
}

