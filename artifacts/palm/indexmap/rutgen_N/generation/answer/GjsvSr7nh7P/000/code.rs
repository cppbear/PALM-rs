// Answer 0

#[test]
fn test_shift_insert_new_value() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    assert_eq!(set.get_index_of(&'*'), None);
    assert_eq!(set.shift_insert(10, '*'), true);
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_move_existing_value() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    assert_eq!(set.shift_insert(10, 'a'), false);
    assert_eq!(set.get_index_of(&'a'), Some(10));
    assert_eq!(set.get_index_of(&'*'), Some(9));
}

#[test]
fn test_shift_insert_move_down_existing_value() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    assert_eq!(set.shift_insert(9, 'z'), false);
    assert_eq!(set.get_index_of(&'z'), Some(9));
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_move_to_len_minus_one() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    assert_eq!(set.len(), 27);
    assert_eq!(set.shift_insert(set.len() - 1, '*'), false);
    assert_eq!(set.get_index_of(&'*'), Some(26));
}

#[test]
fn test_shift_insert_at_len() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    assert_eq!(set.shift_insert(set.len(), '+'), true);
    assert_eq!(set.get_index_of(&'+'), Some(27));
    assert_eq!(set.len(), 28);
}

#[should_panic]
fn test_shift_insert_out_of_bounds_move() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    set.shift_insert(set.len(), 'a');
}

