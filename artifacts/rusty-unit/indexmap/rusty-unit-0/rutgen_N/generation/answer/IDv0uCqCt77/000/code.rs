// Answer 0

#[test]
fn test_sorted_unstable_by_empty_set() {
    let set: indexmap::IndexSet<u32> = indexmap::IndexSet::new();
    let result: Vec<_> = set.sorted_unstable_by(|a, b| a.cmp(b)).collect();
    assert!(result.is_empty());
}

#[test]
fn test_sorted_unstable_by_single_element() {
    let mut set = indexmap::IndexSet::new();
    set.insert(5);
    let result: Vec<_> = set.sorted_unstable_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![5]);
}

#[test]
fn test_sorted_unstable_by_multiple_elements() {
    let mut set = indexmap::IndexSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);
    let result: Vec<_> = set.sorted_unstable_by(|a, b| a.cmp(b)).collect();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_sorted_unstable_by_reverse_order() {
    let mut set = indexmap::IndexSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);
    let result: Vec<_> = set.sorted_unstable_by(|a, b| b.cmp(a)).collect();
    assert_eq!(result, vec![3, 2, 1]);
}

