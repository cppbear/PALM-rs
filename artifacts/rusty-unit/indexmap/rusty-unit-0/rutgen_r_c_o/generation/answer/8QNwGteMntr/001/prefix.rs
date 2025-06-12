// Answer 0

#[test]
fn test_swap_remove_index_valid_lower_bound() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_valid_middle() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    map.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_valid_upper_bound() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.swap_remove_index(1);
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_too_high() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.swap_remove_index(1);
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_negative() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.swap_remove_index(!0);
}

#[test]
fn test_swap_remove_index_multiple_elements() {
    let mut map = IndexMap::new();
    for i in 0..10 {
        map.insert(i, i.to_string());
    }
    for i in 0..10 {
        map.swap_remove_index(0);
    }
}

