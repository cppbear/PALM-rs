// Answer 0

#[test]
fn test_sort_unstable_empty_set() {
    let mut set: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    set.sort_unstable();
    assert!(set.is_empty());
}

#[test]
fn test_sort_unstable_single_element() {
    let mut set = indexmap::IndexSet::from([(1, ())]);
    set.sort_unstable();
    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1]);
}

#[test]
fn test_sort_unstable_two_elements_unsorted() {
    let mut set = indexmap::IndexSet::from([(3, ()), (1, ())]);
    set.sort_unstable();
    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &3]);
}

#[test]
fn test_sort_unstable_two_elements_sorted() {
    let mut set = indexmap::IndexSet::from([(1, ()), (3, ())]);
    set.sort_unstable();
    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &3]);
}

#[test]
fn test_sort_unstable_multiple_elements() {
    let mut set = indexmap::IndexSet::from([(3, ()), (1, ()), (2, ())]);
    set.sort_unstable();
    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
}

#[test]
fn test_sort_unstable_duplicates() {
    let mut set = indexmap::IndexSet::from([(2, ()), (1, ()), (2, ())]);
    set.sort_unstable();
    assert_eq!(set.iter().collect::<Vec<_>>(), vec![&1, &2]);
}

