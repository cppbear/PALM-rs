// Answer 0

#[test]
fn test_sort_empty_set() {
    use indexmap::IndexMap;

    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.sort();
    assert!(set.is_empty());
}

#[test]
fn test_sort_single_element_set() {
    use indexmap::IndexMap;

    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(1, ());
    set.sort();
    assert_eq!(set.keys().cloned().collect::<Vec<_>>(), vec![1]);
}

#[test]
fn test_sort_multiple_elements_set() {
    use indexmap::IndexMap;

    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(3, ());
    set.insert(1, ());
    set.insert(2, ());
    set.sort();
    assert_eq!(set.keys().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_sort_stable_with_duplicate_insertions() {
    use indexmap::IndexMap;

    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(2, ());
    set.insert(3, ());
    set.insert(1, ());
    set.insert(2, ()); // Duplicate, should not affect the sort
    set.sort();
    assert_eq!(set.keys().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
}

