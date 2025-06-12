// Answer 0

#[test]
fn test_splice_with_valid_range_and_replacement() {
    use crate::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = vec![5, 4, 3, 2, 1];
    let removed: Vec<_> = set.splice(2..4, new).collect();

    // Check the contents of the set after splice
    assert!(set.into_iter().eq([0, 1, 5, 3, 2, 4]));
    assert_eq!(removed, &[2, 3]);
}

#[test]
#[should_panic]
fn test_splice_with_out_of_bounds_range() {
    use crate::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = vec![5, 4];
    // This should panic as the start of the range is greater than the end
    let _ = set.splice(4..2, new);
}

#[test]
#[should_panic]
fn test_splice_with_negative_range() {
    use crate::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = vec![5, 4];
    // This should panic as the range is invalid (negative index)
    let _ = set.splice(..1, new);
}

#[test]
fn test_splice_with_empty_replacement() {
    use crate::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let removed: Vec<_> = set.splice(1..3, vec![]).collect();

    // Check that elements from index 1 to 3 were removed
    assert!(set.into_iter().eq([0, 3, 4]));
    assert_eq!(removed, &[1, 2]);
}

