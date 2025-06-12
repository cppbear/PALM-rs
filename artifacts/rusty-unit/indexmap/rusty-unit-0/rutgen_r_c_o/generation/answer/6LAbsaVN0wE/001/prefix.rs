// Answer 0

#[test]
fn test_symmetric_difference_no_elements() {
    let set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let _ = set1.symmetric_difference(&set2);
}

#[test]
fn test_symmetric_difference_one_element_each() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    set1.insert(1);
    set2.insert(2);
    let _ = set1.symmetric_difference(&set2);
}

#[test]
fn test_symmetric_difference_some_common_elements() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    set1.insert(1);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);
    let _ = set1.symmetric_difference(&set2);
}

#[test]
fn test_symmetric_difference_larger_sets() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    
    for i in 0..500 {
        set1.insert(i);
    }
    for i in 250..750 {
        set2.insert(i);
    }
    let _ = set1.symmetric_difference(&set2);
}

#[test]
fn test_symmetric_difference_empty_set_one_non_empty() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    
    set2.insert(42);
    let _ = set1.symmetric_difference(&set2);
}

#[test]
fn test_symmetric_difference_large_values() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    
    for i in 0..1000 {
        set1.insert(i);
    }
    for i in 500..1500 {
        set2.insert(i);
    }
    let _ = set1.symmetric_difference(&set2);
}

