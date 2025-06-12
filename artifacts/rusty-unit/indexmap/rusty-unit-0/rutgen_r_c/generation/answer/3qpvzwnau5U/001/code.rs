// Answer 0

#[test]
fn test_shift_remove_index_valid_removal() {
    let mut set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    
    // Adding elements to the set
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    // Ensure the length is correct before removal
    assert_eq!(set.map.len(), 3);
    
    // Remove an element at a valid index (1)
    let removed_value = set.shift_remove_index(1);
    assert_eq!(removed_value, Some(2));
    
    // Check length after removal
    assert_eq!(set.map.len(), 2);
}

#[test]
fn test_shift_remove_index_on_empty_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    
    // Removing from an empty set should return None
    let removed_value = set.shift_remove_index(0);
    assert_eq!(removed_value, None);
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    let mut set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };
    
    // Adding elements to the set
    set.map.insert(1, ());
    set.map.insert(2, ());

    // Remove an element at an invalid index (2, which is out of bounds)
    let removed_value = set.shift_remove_index(2);
    assert_eq!(removed_value, None);
}

#[test]
fn test_shift_remove_index_multiple_removals() {
    let mut set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap::new(),
    };

    // Adding elements to the set
    set.map.insert(10, ());
    set.map.insert(20, ());
    set.map.insert(30, ());

    // Initial length
    assert_eq!(set.map.len(), 3);

    // Remove first element
    let removed_first = set.shift_remove_index(0);
    assert_eq!(removed_first, Some(10));
    assert_eq!(set.map.len(), 2);
    
    // Remove the new first element (previously second)
    let removed_second = set.shift_remove_index(0);
    assert_eq!(removed_second, Some(20));
    assert_eq!(set.map.len(), 1);

    // Finally remove the last remaining element
    let removed_last = set.shift_remove_index(0);
    assert_eq!(removed_last, Some(30));
    assert_eq!(set.map.len(), 0);
}

