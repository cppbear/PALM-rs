// Answer 0

#[test]
fn test_bitand_non_empty_intersection() {
    use indexmap::IndexSet;

    // Create two IndexSets with some overlapping values
    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![3, 4, 5, 6]);

    // Perform the intersection using the bitand function
    let result = set_a.bitand(&set_b);

    // The intersection should be [3, 4]
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![3, 4]);
    assert_eq!(result, expected);
}

#[test]
fn test_bitand_empty_intersection() {
    use indexmap::IndexSet;

    // Create two IndexSets with no overlapping values
    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![3, 4]);

    // Perform the intersection using the bitand function
    let result = set_a.bitand(&set_b);

    // The intersection should be empty
    let expected: IndexSet<i32> = IndexSet::new();
    assert_eq!(result, expected);
}

#[test]
fn test_bitand_identical_sets() {
    use indexmap::IndexSet;

    // Create two identical IndexSets
    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);

    // Perform the intersection using the bitand function
    let result = set_a.bitand(&set_b);

    // The result should be the same as the sets
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    assert_eq!(result, expected);
}

#[test]
fn test_bitand_one_empty_set() {
    use indexmap::IndexSet;

    // Create one empty IndexSet
    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2]);
    let set_b: IndexSet<i32> = IndexSet::new();

    // Perform the intersection using the bitand function
    let result = set_a.bitand(&set_b);

    // The intersection should be empty
    let expected: IndexSet<i32> = IndexSet::new();
    assert_eq!(result, expected);
}

