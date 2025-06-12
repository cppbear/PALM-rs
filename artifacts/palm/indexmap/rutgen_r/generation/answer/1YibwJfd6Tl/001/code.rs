// Answer 0

#[test]
fn test_splice_valid_range() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 6, 7];
    let removed: Vec<_> = set.splice(1..3, new).collect();

    assert!(set.into_iter().eq([0, 5, 6, 7, 3, 4]));
    assert_eq!(removed, &[1, 2]);
}

#[test]
fn test_splice_empty_replace() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new: Vec<i32> = Vec::new();
    let removed: Vec<_> = set.splice(1..3, new).collect();

    assert!(set.into_iter().eq([0, 3, 4]));
    assert_eq!(removed, &[1, 2]);
}

#[test]
fn test_splice_same_length() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 6];
    let removed: Vec<_> = set.splice(2..4, new).collect();

    assert!(set.into_iter().eq([0, 1, 5, 6, 4]));
    assert_eq!(removed, &[2, 3]);
}

#[should_panic]
fn test_splice_start_greater_than_end() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 6];
    let _: Vec<_> = set.splice(3..2, new).collect();
}

#[should_panic]
fn test_splice_end_out_of_bounds() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3]);
    let new = [5];
    let _: Vec<_> = set.splice(1..5, new).collect();
}

