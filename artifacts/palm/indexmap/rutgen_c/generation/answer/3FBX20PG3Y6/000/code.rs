// Answer 0

#[test]
fn test_intersection_new() {
    use std::collections::hash_map::RandomState;

    // Define two IndexSets with the same type and hasher
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());

    // Create an Intersection from the two sets
    let intersection = Intersection::new(&set1, &set2);

    // Check that the Intersection has been created with the correct references
    assert_eq!(intersection.other, &set2);
}

#[test]
fn test_intersection_new_empty_sets() {
    use std::collections::hash_map::RandomState;

    // Define two empty IndexSets
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    // Create an Intersection from the two empty sets
    let intersection = Intersection::new(&set1, &set2);

    // Check that the Intersection has been created with the correct references
    assert_eq!(intersection.other, &set2);
}

