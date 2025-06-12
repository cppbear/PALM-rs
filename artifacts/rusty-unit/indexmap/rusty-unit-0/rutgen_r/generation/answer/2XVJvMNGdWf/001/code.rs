// Answer 0

#[test]
fn test_bitxor_with_non_overlapping_sets() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![4, 5, 6]);
    
    let result = set_a.bitxor(&set_b);
    
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4, 5, 6]);
    
    assert_eq!(result, expected);
}

#[test]
fn test_bitxor_with_partial_overlapping_sets() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![3, 4, 5]);

    let result = set_a.bitxor(&set_b);
    
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 4, 5]);
    
    assert_eq!(result, expected);
}

#[test]
fn test_bitxor_with_identical_sets() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);

    let result = set_a.bitxor(&set_b);
    
    let expected: IndexSet<i32> = IndexSet::new();
    
    assert_eq!(result, expected);
}

#[test]
fn test_bitxor_with_empty_sets() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::new();
    let set_b: IndexSet<i32> = IndexSet::new();

    let result = set_a.bitxor(&set_b);
    
    let expected: IndexSet<i32> = IndexSet::new();
    
    assert_eq!(result, expected);
}

#[test]
fn test_bitxor_with_empty_and_non_empty_set() {
    use indexmap::IndexSet;

    let set_a: IndexSet<i32> = IndexSet::new();
    let set_b: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);

    let result = set_a.bitxor(&set_b);
    
    let expected: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
    
    assert_eq!(result, expected);
}

