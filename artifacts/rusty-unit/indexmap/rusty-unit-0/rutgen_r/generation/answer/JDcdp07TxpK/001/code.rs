// Answer 0

#[test]
fn test_sorted_by_empty_set() {
    let set: IndexMap<i32, ()> = IndexMap::new();
    let result: Vec<_> = set.sorted_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![]);
}

#[test]
fn test_sorted_by_single_element() {
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(1, ());
    let result: Vec<_> = set.sorted_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_sorted_by_two_elements_sorted() {
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(2, ());
    set.insert(1, ());
    let result: Vec<_> = set.sorted_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_sorted_by_two_elements_reverse_sorted() {
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(1, ());
    set.insert(2, ());
    let result: Vec<_> = set.sorted_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_sorted_by_multiple_elements() {
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(3, ());
    set.insert(1, ());
    set.insert(2, ());
    let result: Vec<_> = set.sorted_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_sorted_by_with_duplicate_entries() {
    let mut set: IndexMap<i32, ()> = IndexMap::new();
    set.insert(2, ());
    set.insert(1, ());
    set.insert(2, ());
    let result: Vec<_> = set.sorted_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![1, 2, 2]);
}

