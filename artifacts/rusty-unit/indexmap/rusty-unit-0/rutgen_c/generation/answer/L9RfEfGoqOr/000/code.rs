// Answer 0

#[test]
fn test_union_new_with_different_hashers() {
    use std::collections::hash_map::RandomState;
    use std::collections::hash_set::HashSet;
    
    // Create two IndexSet instances with different hashers
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    // Populate set1 and set2
    set1.insert(1);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);

    // Create the union
    let union = Union::new(&set1, &set2);

    // Test the union properties
    let union_elements: Vec<_> = union.iter.collect();
    assert_eq!(union_elements, vec![1, 2, 3]);
}

#[test]
fn test_union_new_empty_sets() {
    use std::collections::hash_map::RandomState;
    
    // Create two empty IndexSet instances
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    // Create the union
    let union = Union::new(&set1, &set2);
  
    // Test the union properties
    let union_elements: Vec<_> = union.iter.collect();
    assert_eq!(union_elements, Vec::<i32>::new());
}

#[test]
fn test_union_new_with_some_common_elements() {
    use std::collections::hash_map::RandomState;
    
    // Create two IndexSet instances with some common elements
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    // Populate both sets
    set1.insert(1);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);
    
    // Create the union
    let union = Union::new(&set1, &set2);

    // Test the union properties
    let union_elements: Vec<_> = union.iter.collect();
    assert_eq!(union_elements, vec![1, 2, 3]);
}

