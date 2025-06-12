// Answer 0

#[test]
fn test_swap_indices_valid() {
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert("value1");
    set.insert("value2");
    set.insert("value3");

    // Swap valid indices
    set.swap_indices(0, 1);
    assert_eq!(set.get_index(0).unwrap(), "value2");
    assert_eq!(set.get_index(1).unwrap(), "value1");
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_a() {
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert("value1");
    set.insert("value2");

    // Attempt to swap with an out of bounds index
    set.swap_indices(2, 1);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_b() {
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert("value1");
    set.insert("value2");

    // Attempt to swap with an out of bounds index
    set.swap_indices(0, 2);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_both() {
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    
    // Attempt to swap both indices out of bounds
    set.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_no_op() {
    use indexmap::IndexSet;

    let mut set = IndexSet::new();
    set.insert("value1");
    set.insert("value2");

    // Swap the same index (should not change anything)
    set.swap_indices(0, 0);
    assert_eq!(set.get_index(0).unwrap(), "value1");
    assert_eq!(set.get_index(1).unwrap(), "value2");
}

