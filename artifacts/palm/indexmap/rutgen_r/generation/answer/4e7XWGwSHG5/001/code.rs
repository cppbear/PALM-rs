// Answer 0

#[test]
fn test_sort_unstable_by_basic() {
    let mut set = indexmap::IndexSet::from([3, 1, 2]);
    set.sort_unstable_by(|a, b| a.cmp(b));
    let expected: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected, vec![1, 2, 3]);
}

#[test]
fn test_sort_unstable_by_empty() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.sort_unstable_by(|a, b| a.cmp(b));
    let expected: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected, Vec::<i32>::new());
}

#[test]
fn test_sort_unstable_by_reverse_order() {
    let mut set = indexmap::IndexSet::from([5, 4, 3]);
    set.sort_unstable_by(|a, b| a.cmp(b));
    let expected: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected, vec![3, 4, 5]);
}

#[test]
fn test_sort_unstable_by_identical_elements() {
    let mut set = indexmap::IndexSet::from([1, 1, 1]);
    set.sort_unstable_by(|a, b| a.cmp(b));
    let expected: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected, vec![1, 1, 1]);
}

#[test]
fn test_sort_unstable_by_floats() {
    let mut set = indexmap::IndexSet::from([1.1, 2.2, 0.0, -1.1]);
    set.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let expected: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected, vec![-1.1, 0.0, 1.1, 2.2]);
}

#[should_panic]
fn test_sort_unstable_by_panic() {
    let mut set = indexmap::IndexSet::from([1, 2, 3]);
    set.sort_unstable_by(|_, _| panic!("This is a panic test"));
}

