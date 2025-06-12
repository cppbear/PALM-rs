// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.swap_remove_index(1);
    assert_eq!(result, Some((2, "b")));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&3), Some(&"c"));
}

#[test]
fn test_swap_remove_index_first_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "first");
    map.insert(2, "second");

    let result = map.swap_remove_index(0);
    assert_eq!(result, Some((1, "first")));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&2), Some(&"second"));
}

#[test]
fn test_swap_remove_index_last_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "only");

    let result = map.swap_remove_index(0);
    assert_eq!(result, Some((1, "only")));
    assert!(map.is_empty());
}

#[should_panic]
fn test_swap_remove_index_out_of_bounds_negative() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "a");

    map.swap_remove_index(usize::MAX); // Out of bounds
}

#[should_panic]
fn test_swap_remove_index_out_of_bounds_too_high() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "x");

    map.swap_remove_index(1); // Out of bounds
}

