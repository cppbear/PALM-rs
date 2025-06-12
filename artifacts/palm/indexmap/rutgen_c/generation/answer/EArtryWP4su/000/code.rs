// Answer 0

#[test]
fn test_set_difference_empty_sets() {
    let set_a: super::IndexSet<i32, std::collections::hash_map::RandomState> = Default::default();
    let set_b: super::IndexSet<i32, std::collections::hash_map::RandomState> = Default::default();
    
    let difference = &set_a - &set_b;
    
    assert_eq!(difference.len(), 0);
}

#[test]
fn test_set_difference_non_empty_sets() {
    let mut set_a = super::IndexSet::default();
    let mut set_b = super::IndexSet::default();
    
    set_a.extend(vec![1, 2, 3]);
    set_b.extend(vec![2, 3, 4]);
    
    let difference = &set_a - &set_b;
    
    let expected: super::IndexSet<i32, std::collections::hash_map::RandomState> = vec![1].into_iter().collect();
    assert_eq!(difference, expected);
}

#[test]
fn test_set_difference_all_elements_in_other() {
    let mut set_a = super::IndexSet::default();
    let mut set_b = super::IndexSet::default();
    
    set_a.extend(vec![1, 2, 3]);
    set_b.extend(vec![1, 2, 3]);
    
    let difference = &set_a - &set_b;
    
    assert_eq!(difference.len(), 0);
}

#[test]
fn test_set_difference_with_unique_elements() {
    let mut set_a = super::IndexSet::default();
    let mut set_b = super::IndexSet::default();
    
    set_a.extend(vec![1, 2, 3]);
    set_b.extend(vec![4, 5, 6]);
    
    let difference = &set_a - &set_b;
    
    let expected: super::IndexSet<i32, std::collections::hash_map::RandomState> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(difference, expected);
}

