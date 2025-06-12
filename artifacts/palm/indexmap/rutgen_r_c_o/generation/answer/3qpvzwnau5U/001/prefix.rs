// Answer 0

#[test]
fn test_shift_remove_index_valid_index() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(10);
    index_set.insert(20);
    index_set.insert(30);
    
    let _ = index_set.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_first_index() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(10);
    index_set.insert(20);
    index_set.insert(30);
    
    let _ = index_set.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_last_index() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(10);
    index_set.insert(20);
    index_set.insert(30);
    
    let _ = index_set.shift_remove_index(2);
}

#[should_panic]
fn test_shift_remove_index_out_of_bounds_negative() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(10);
    
    let _ = index_set.shift_remove_index(-1);
}

#[should_panic]
fn test_shift_remove_index_out_of_bounds_too_large() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(10);
    
    let _ = index_set.shift_remove_index(1);
}

