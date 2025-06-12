// Answer 0

#[test]
fn test_swap_indices_valid() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    map.swap_indices(0, 2);

    let keys: Vec<&str> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["key3", "key2", "key1"]);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_a() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    map.swap_indices(2, 1); // a is out of bounds
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_b() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    map.swap_indices(0, 2); // b is out of bounds
}

#[test]
fn test_swap_indices_no_operation() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    map.swap_indices(0, 0); // swapping the same index

    let keys: Vec<&str> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["key1", "key2"]);
}

