// Answer 0

#[test]
fn test_splice_valid_range() {
    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 4, 3, 2, 1];
    let removed: Vec<_> = set.splice(2..4, new).collect();
    
    assert!(set.into_iter().eq([0, 1, 5, 3, 2, 4]));
    assert_eq!(removed, &[2, 3]);
}

#[test]
#[should_panic(expected = "panicked at 'start point is greater than the end point'")]
fn test_splice_invalid_range_start_greater_than_end() {
    let mut set = IndexSet::from([0, 1, 2, 3, 4]);
    let new = [5, 6];
    set.splice(4..2, new);
}

#[test]
#[should_panic(expected = "panicked at 'end point is greater than the length of the set'")]
fn test_splice_invalid_end_point() {
    let mut set = IndexSet::from([0, 1, 2]);
    let new = [5, 6, 7];
    set.splice(1..5, new);
}

#[test]
fn test_splice_empty_replace_with() {
    let mut set = IndexSet::from([0, 1, 2, 3]);
    let new: Vec<i32> = vec![];
    let removed: Vec<_> = set.splice(1..3, new).collect();
    
    assert!(set.into_iter().eq([0, 3]));
    assert_eq!(removed, &[1, 2]);
}

#[test]
fn test_splice_no_change_when_replacing_with_existing_values() {
    let mut set = IndexSet::from([0, 1, 2, 3]);
    let new = [2, 3, 4];
    let removed: Vec<_> = set.splice(1..3, new).collect();
    
    assert!(set.into_iter().eq([0, 2, 3, 4]));
    assert_eq!(removed, &[1, 2]);
}

