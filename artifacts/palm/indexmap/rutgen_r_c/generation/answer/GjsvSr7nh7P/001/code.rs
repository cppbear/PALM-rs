// Answer 0

#[test]
fn test_shift_insert_new_value() {
    let mut set: super::IndexSet<char, std::collections::hash_map::RandomState> = ('a'..='z').collect();
    
    // Inserting a new value at a valid index
    assert_eq!(set.get_index_of(&'*'), None);
    assert_eq!(set.shift_insert(10, '*'), true);
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_move_existing_value_up() {
    let mut set: super::IndexSet<char, std::collections::hash_map::RandomState> = ('a'..='z').collect();
    
    // Moving the value 'a' up to 10 will shift others down
    assert_eq!(set.shift_insert(10, 'a'), false);
    assert_eq!(set.get_index_of(&'a'), Some(10));
    assert_eq!(set.get_index_of(&'*'), Some(9));
}

#[test]
fn test_shift_insert_move_existing_value_down() {
    let mut set: super::IndexSet<char, std::collections::hash_map::RandomState> = ('a'..='z').collect();
    
    // Moving the value 'z' down to 9 will shift others up
    assert_eq!(set.shift_insert(9, 'z'), false);
    assert_eq!(set.get_index_of(&'z'), Some(9));
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_existing_value_move_limit() {
    let mut set: super::IndexSet<char, std::collections::hash_map::RandomState> = ('a'..='z').collect();
    
    // Moving to len-1, should not move existing value
    assert_eq!(set.len(), 27);
    assert_eq!(set.shift_insert(set.len() - 1, '*'), false);
    assert_eq!(set.get_index_of(&'*'), Some(26));
}

#[test]
fn test_shift_insert_new_value_at_end() {
    let mut set: super::IndexSet<char, std::collections::hash_map::RandomState> = ('a'..='z').collect();
    
    // Insert a new value at the end
    assert_eq!(set.shift_insert(set.len(), '+'), true);
    assert_eq!(set.get_index_of(&'+'), Some(27));
    assert_eq!(set.len(), 28);
}

#[should_panic]
#[test]
fn test_shift_insert_panic_moving_existing_value_out_of_bounds() {
    let mut set: super::IndexSet<char, std::collections::hash_map::RandomState> = ('a'..='z').collect();
    
    // Invalid index for moving an existing value
    set.shift_insert(set.len(), 'a');
}

