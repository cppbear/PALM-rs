// Answer 0

#[test]
fn test_shift_insert_new_value_within_bounds() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Insert a new value '*' at a valid index (10).
    assert_eq!(set.get_index_of(&'*'), None);
    assert_eq!(set.shift_insert(10, '*'), true);
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_move_existing_value_up() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Move existing value 'a' to index 10, shifts '*' down.
    assert_eq!(set.shift_insert(10, 'a'), false);
    assert_eq!(set.get_index_of(&'a'), Some(10));
    assert_eq!(set.get_index_of(&'*'), Some(9));
}

#[test]
fn test_shift_insert_move_existing_value_down() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Move existing value 'z' to index 9, shifts '*' up.
    assert_eq!(set.shift_insert(9, 'z'), false);
    assert_eq!(set.get_index_of(&'z'), Some(9));
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_new_value_at_endpoint() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Valid insertion at the end of the set.
    assert_eq!(set.len(), 27);
    assert_eq!(set.shift_insert(set.len(), '+'), true);
    assert_eq!(set.get_index_of(&'+'), Some(27));
    assert_eq!(set.len(), 28);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_shift_insert_move_existing_value_out_of_bounds() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    // This should panic: invalid index for moving an existing value.
    set.shift_insert(set.len(), 'a');
}

#[test]
fn test_shift_insert_insert_with_invalid_index() {
    use indexmap::IndexSet;
    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Test inserting at out of bounds index (index 30).
    let result = set.shift_insert(30, '*');
    assert_eq!(result, true);
    assert_eq!(set.get_index_of(&'*'), Some(30));
}

