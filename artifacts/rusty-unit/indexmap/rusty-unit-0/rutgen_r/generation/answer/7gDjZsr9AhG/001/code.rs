// Answer 0

#[test]
fn test_swap_remove_full_existing_element() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    // Initialize an IndexSet with some elements
    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    // Test removing an existing element
    let result = set.swap_remove_full(&2);
    assert_eq!(result, Some((1, 2))); // 1 is the index of the removed element (2)

    // Verify the size of the set after operation
    assert_eq!(set.len(), 2);
    assert!(!set.contains(&2));
}

#[test]
fn test_swap_remove_full_non_existing_element() {
    use indexmap::IndexSet;

    // Initialize an IndexSet with some elements
    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    
    // Attempt to remove a non-existing element
    let result = set.swap_remove_full(&3);
    assert_eq!(result, None); // Should return None since 3 is not in the set

    // Verify the size of the set after operation
    assert_eq!(set.len(), 2);
}

#[test]
fn test_swap_remove_full_edge_case_empty_set() {
    use indexmap::IndexSet;

    // Initialize an empty IndexSet
    let mut set: IndexSet<i32> = IndexSet::new();
    
    // Attempt to remove from an empty set
    let result = set.swap_remove_full(&1);
    assert_eq!(result, None); // Should return None since the set is empty
}

#[test]
fn test_swap_remove_full_larger_set() {
    use indexmap::IndexSet;

    // Initialize a larger IndexSet with multiple elements
    let mut set = IndexSet::new();
    set.extend(1..=10); // Insert elements 1 to 10

    // Test removing an element in the middle
    let result_mid = set.swap_remove_full(&5);
    assert_eq!(result_mid, Some((4, 5))); // 4 is the index of the removed element (5)

    // Test removing the last element
    let result_last = set.swap_remove_full(&10);
    assert_eq!(result_last, Some((8, 10))); // 8 is the index of the removed element (10)

    // Verify the size of the set after operations
    assert_eq!(set.len(), 8);
    assert!(!set.contains(&5));
    assert!(!set.contains(&10));
}

