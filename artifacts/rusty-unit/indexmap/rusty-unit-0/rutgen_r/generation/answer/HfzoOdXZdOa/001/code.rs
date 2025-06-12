// Answer 0

#[test]
fn test_truncate_with_zero_length() {
    let mut set = indexmap::IndexSet::from_iter(vec![1, 2, 3, 4, 5]);
    set.truncate(0);
    assert!(set.is_empty());
}

#[test]
fn test_truncate_with_length_greater_than_set() {
    let mut set = indexmap::IndexSet::from_iter(vec![1, 2, 3]);
    set.truncate(5);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_truncate_with_length_equal_to_set() {
    let mut set = indexmap::IndexSet::from_iter(vec![1, 2, 3]);
    set.truncate(3);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_truncate_with_length_one() {
    let mut set = indexmap::IndexSet::from_iter(vec![1, 2, 3]);
    set.truncate(1);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&1));
}

#[test]
fn test_truncate_with_empty_set() {
    let mut set = indexmap::IndexSet::<i32>::new();
    set.truncate(2);
    assert!(set.is_empty());
}

