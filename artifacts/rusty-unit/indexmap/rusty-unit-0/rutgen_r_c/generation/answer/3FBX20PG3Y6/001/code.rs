// Answer 0

#[test]
fn test_intersection_new() {
    use std::collections::hash_map::RandomState;

    // Create two IndexSet instances
    let set1: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let set2: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());

    // Initialize some elements
    let mut set1 = set1;
    let mut set2 = set2;

    set1.reserve(3);
    set1.extend(vec![1, 2, 3]);
    
    set2.reserve(3);
    set2.extend(vec![3, 4, 5]);

    // Create Intersection instance
    let intersection = Intersection::new(&set1, &set2);

    // Ensure the intersection is created correctly
    assert!(intersection.other == &set2);
    assert!(intersection.iter.len() == 1); // Evaluate based on expected intersecting elements
}
    
#[test]
fn test_intersection_new_empty_sets() {
    // Create two empty IndexSet instances
    let set1: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    // Create Intersection instance
    let intersection = Intersection::new(&set1, &set2);

    // Ensure the intersection is created correctly with empty sets
    assert!(intersection.other == &set2);
    assert!(intersection.iter.len() == 0);
}

#[test]
#[should_panic]
fn test_intersection_new_with_invalid_reference() {
    // Initialize IndexSet instances but drop one before creating the intersection
    let set1: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let set2: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let _dropped_set1 = set1;

    // Attempt to create an intersection with a dropped reference, which should panic
    let _intersection = Intersection::new(&_dropped_set1, &set2);
}

