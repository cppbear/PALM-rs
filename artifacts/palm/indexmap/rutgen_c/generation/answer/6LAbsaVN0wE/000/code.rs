// Answer 0

#[test]
fn test_symmetric_difference_empty_sets() {
    use std::collections::hash_map::RandomState;
    let set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    
    let sym_diff = set1.symmetric_difference(&set2);
    let result: Vec<_> = sym_diff.collect();
    
    assert!(result.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_set() {
    use std::collections::hash_map::RandomState;
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    set1.insert(1);
    set1.insert(2);

    let set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    let sym_diff = set1.symmetric_difference(&set2);
    let result: Vec<_> = sym_diff.collect();
    
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_symmetric_difference_no_common_elements() {
    use std::collections::hash_map::RandomState;
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    
    set1.insert(1);
    set1.insert(2);
    set2.insert(3);
    set2.insert(4);
    
    let sym_diff = set1.symmetric_difference(&set2);
    let result: Vec<_> = sym_diff.collect();
    
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_symmetric_difference_with_common_elements() {
    use std::collections::hash_map::RandomState;
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);
    
    let sym_diff = set1.symmetric_difference(&set2);
    let result: Vec<_> = sym_diff.collect();
    
    assert_eq!(result, vec![1, 4]);
}

