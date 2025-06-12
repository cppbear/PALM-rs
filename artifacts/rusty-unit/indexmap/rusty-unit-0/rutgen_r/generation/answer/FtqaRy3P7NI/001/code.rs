// Answer 0

#[test]
fn test_difference_no_overlap() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![4, 5, 6]);
    
    let result: Vec<_> = set1.difference(&set2).collect();
    
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_difference_partial_overlap() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3, 4]);
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![3, 4, 5, 6]);
    
    let result: Vec<_> = set1.difference(&set2).collect();
    
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_difference_all_overlap() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    
    let result: Vec<_> = set1.difference(&set2).collect();
    
    assert!(result.is_empty());
}

#[test]
fn test_difference_empty_set2() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    let set2: IndexSet<i32, RandomState> = IndexSet::new();
    
    let result: Vec<_> = set1.difference(&set2).collect();
    
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_difference_empty_set1() {
    use std::collections::hash_map::RandomState;
    use indexmap::indexset::IndexSet;

    let set1: IndexSet<i32, RandomState> = IndexSet::new();
    let set2: IndexSet<i32, RandomState> = IndexSet::from_iter(vec![1, 2, 3]);
    
    let result: Vec<_> = set1.difference(&set2).collect();
    
    assert!(result.is_empty());
}

