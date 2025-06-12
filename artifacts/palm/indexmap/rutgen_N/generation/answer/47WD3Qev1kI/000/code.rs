// Answer 0

#[test]
fn test_insert_before_with_new_value() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    assert_eq!(set.get_index_of(&'*'), None);
    assert_eq!(set.insert_before(10, '*'), (10, true));
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_insert_before_with_existing_value_move_up() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    assert_eq!(set.insert_before(10, 'a'), (9, false));
    assert_eq!(set.get_index_of(&'a'), Some(9));
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_insert_before_with_existing_value_move_down() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    assert_eq!(set.insert_before(10, 'z'), (10, false));
    assert_eq!(set.get_index_of(&'z'), Some(10));
    assert_eq!(set.get_index_of(&'*'), Some(11));
}

#[test]
fn test_insert_before_at_endpoint() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    assert_eq!(set.len(), 27);
    assert_eq!(set.insert_before(set.len(), '*'), (26, false));
    assert_eq!(set.get_index_of(&'*'), Some(26));
    
    assert_eq!(set.insert_before(set.len(), '+'), (27, true));
    assert_eq!(set.get_index_of(&'+'), Some(27));
    assert_eq!(set.len(), 28);
}

#[test]
#[should_panic]
fn test_insert_before_out_of_bounds() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    set.insert_before(28, '*'); // This should panic
}

