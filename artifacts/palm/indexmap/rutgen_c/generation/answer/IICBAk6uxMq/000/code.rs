// Answer 0

#[test]
fn test_swap_indices_same_index() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket::new(0)); // Assuming Bucket has a new method
    map.entries.push(Bucket::new(1));

    map.swap_indices(0, 0);
    assert_eq!(map.entries[0], Bucket::new(0)); // No change expected
}

#[test]
fn test_swap_indices_different_indices() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket::new(0)); // Assuming Bucket has a new method
    map.entries.push(Bucket::new(1));

    map.swap_indices(0, 1);
    assert_eq!(map.entries[0], Bucket::new(1)); // First index should now be second
    assert_eq!(map.entries[1], Bucket::new(0)); // Second index should now be first
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_lower() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket::new(0)); // Assuming Bucket has a new method

    map.swap_indices(0, 1); // Second index is out of bounds
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_upper() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket::new(0)); // Assuming Bucket has a new method

    map.swap_indices(1, 0); // First index is out of bounds
}

