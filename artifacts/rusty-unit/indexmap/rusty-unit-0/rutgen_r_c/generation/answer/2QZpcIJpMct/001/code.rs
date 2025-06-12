// Answer 0

#[test]
fn test_splice_new_valid_range() {
    use std::collections::hash_map::RandomState;

    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);

    let replace_with = vec![4, 5].into_iter();
    let splice = Splice::new(&mut index_set, 0..2, replace_with);

    assert_eq!(index_set.len(), 1); // Expecting 1 left after splicing the first two elements.
    assert!(index_set.contains(&3)); // Should contain the third element.
}

#[test]
#[should_panic]
fn test_splice_new_empty_range() {
    use std::collections::hash_map::RandomState;

    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();

    let replace_with = vec![4, 5].into_iter();
    // Trying to splice with an empty range should panic since there's nothing to replace.
    let _splice = Splice::new(&mut index_set, 0..0, replace_with);
}

#[test]
#[should_panic]
fn test_splice_new_invalid_range_high() {
    use std::collections::hash_map::RandomState;

    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);

    let replace_with = vec![4, 5].into_iter();
    // Attempting to splice a range that is out of bounds should panic.
    let _splice = Splice::new(&mut index_set, 5..10, replace_with);
}

#[test]
fn test_splice_new_full_replacement() {
    use std::collections::hash_map::RandomState;

    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);

    let replace_with = vec![4, 5, 6].into_iter();
    let splice = Splice::new(&mut index_set, .., replace_with);
    
    assert_eq!(index_set.len(), 3); // Expecting full replacement; the code should handle it correctly.
    assert!(index_set.contains(&4));
    assert!(index_set.contains(&5));
    assert!(index_set.contains(&6));
}

