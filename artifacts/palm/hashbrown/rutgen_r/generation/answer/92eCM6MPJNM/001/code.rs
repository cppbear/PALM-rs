// Answer 0

#[test]
fn test_into_keys_with_single_entry() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);

    let mut vec: Vec<&str> = map.into_keys().collect();
    vec.sort_unstable();
    assert_eq!(vec, ["a"]);
}

#[test]
fn test_into_keys_with_multiple_entries() {
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
fn test_into_keys_with_no_entries() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new(); // empty map

    let vec: Vec<&str> = map.into_keys().collect();
    assert!(vec.is_empty());
}

#[test]
#[should_panic]
fn test_into_keys_after_consumption() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);

    let _keys: Vec<&str> = map.into_keys().collect(); // consume the map

    // The map should not be used again after calling into_keys
    let _ = map.get("a"); // This should panic, as the map has been consumed.
}

