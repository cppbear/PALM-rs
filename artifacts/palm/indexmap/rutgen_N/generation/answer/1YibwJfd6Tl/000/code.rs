// Answer 0

#[test]
fn test_splice_inserts_new_elements() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 4, 3, 2, 1];
    let removed: Vec<_> = set.splice(2..4, new).collect();

    assert!(set.into_iter().eq([0, 1, 5, 3, 2, 4]));
    assert_eq!(removed, &[2, 3]);
}

#[test]
#[should_panic(expected = "start index out of bounds")]
fn test_splice_panics_start_greater_than_end() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 6];

    // This should panic because 3 > 2
    let _removed: Vec<_> = set.splice(3..2, new).collect();
}

#[test]
#[should_panic(expected = "end index out of bounds")]
fn test_splice_panics_end_greater_than_length() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 6];

    // This should panic because the end index (6) is greater than the length (5)
    let _removed: Vec<_> = set.splice(2..6, new).collect();
}

#[test]
fn test_splice_replaces_with_existing_elements() {
    use indexmap::IndexSet;

    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [1, 5, 6];
    let removed: Vec<_> = set.splice(1..3, new).collect();

    assert!(set.into_iter().eq([0, 1, 6, 5, 4]));
    assert_eq!(removed, &[1, 2]);
}

