// Answer 0

#[test]
fn test_into_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let mut vec: Vec<&str> = map.into_keys().collect();

    vec.sort_unstable();
    assert_eq!(vec, ["a", "b", "c"]);
}

#[test]
fn test_into_keys_empty() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    let vec: Vec<&str> = map.into_keys().collect();

    assert!(vec.is_empty());
}

#[test]
fn test_into_keys_single() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);

    let mut vec: Vec<&str> = map.into_keys().collect();

    vec.sort_unstable();
    assert_eq!(vec, ["a"]);
}

