// Answer 0

#[test]
fn test_swap_indices_valid() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    map.swap_indices(0, 1); // Swap indices 0 and 1

    assert_eq!(map.get_index(0), Some(&2)); // Index 0 should now be "b"
    assert_eq!(map.get_index(1), Some(&1)); // Index 1 should now be "a"
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_low() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    
    map.swap_indices(0, 1); // Attempting to swap index 0 with a non-existing index
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_high() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    map.swap_indices(1, 2); // Attempting to swap index 1 with a non-existing index
}

#[test]
fn test_swap_indices_no_effect() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    map.swap_indices(0, 0); // Swapping the same index should have no effect

    assert_eq!(map.get_index(0), Some(&1)); // Index 0 should still be "a"
    assert_eq!(map.get_index(1), Some(&2)); // Index 1 should still be "b"
}

