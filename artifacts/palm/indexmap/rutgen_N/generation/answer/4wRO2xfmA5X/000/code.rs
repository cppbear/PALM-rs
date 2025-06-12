// Answer 0

#[test]
fn test_swap_indices_valid() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    // Swap indices 0 and 2
    map.swap_indices(0, 2);

    assert_eq!(map.keys().collect::<Vec<_>>(), vec![3, 2, 1]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds_a() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    // Try to swap index 2 (out of bounds) with index 1
    map.swap_indices(2, 1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds_b() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    // Try to swap index 0 with index 2 (out of bounds)
    map.swap_indices(0, 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_both_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, "one");

    // Try to swap index 1 (out of bounds) with index 1 (out of bounds)
    map.swap_indices(1, 1);
}

