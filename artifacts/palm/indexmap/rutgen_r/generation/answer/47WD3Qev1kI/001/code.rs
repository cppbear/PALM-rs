// Answer 0

#[test]
fn test_insert_before_valid_insertion() {
    use indexmap::IndexSet;

    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Insert '*' at the end (index 10)
    assert_eq!(set.insert_before(10, '*'), (10, true));
    assert_eq!(set.get_index_of(&'*'), Some(10));

    // Move 'a' before index 10
    assert_eq!(set.insert_before(10, 'a'), (9, false));
    assert_eq!(set.get_index_of(&'a'), Some(9));
    assert_eq!(set.get_index_of(&'*'), Some(10));

    // Move 'z' to index 10, should return false and shift '*'
    assert_eq!(set.insert_before(10, 'z'), (10, false));
    assert_eq!(set.get_index_of(&'z'), Some(10));
    assert_eq!(set.get_index_of(&'*'), Some(11));

    // Inserting at the endpoint
    assert_eq!(set.len(), 27);
    assert_eq!(set.insert_before(set.len(), '*'), (26, false));
    assert_eq!(set.get_index_of(&'*'), Some(26));

    // Insert '+' at the end of the set
    assert_eq!(set.insert_before(set.len(), '+'), (27, true));
    assert_eq!(set.get_index_of(&'+'), Some(27));
    assert_eq!(set.len(), 28);
}

#[test]
#[should_panic]
fn test_insert_before_out_of_bounds_lower() {
    use indexmap::IndexSet;

    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    // Trying to insert before index -1, should panic
    set.insert_before((!0) as usize, 'a');
}

#[test]
#[should_panic]
fn test_insert_before_out_of_bounds_upper() {
    use indexmap::IndexSet;

    let mut set: IndexSet<char> = ('a'..='z').collect();
    
    // Trying to insert before index 27, should panic
    set.insert_before(27, 'a');
}

#[test]
fn test_insert_before_with_existing_value() {
    use indexmap::IndexSet;

    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Insert '*' before 'c' (index 2)
    assert_eq!(set.insert_before(2, '*'), (2, true));
    assert_eq!(set.get_index_of(&'*'), Some(2));

    // Now, trying to insert a duplicate '*' before 'e', should return false and shift '*'
    assert_eq!(set.insert_before(4, '*'), (3, false));
    assert_eq!(set.get_index_of(&'*'), Some(3));
    assert_eq!(set.get_index_of(&'e'), Some(4));
}

#[test]
fn test_insert_before_at_zero() {
    use indexmap::IndexSet;

    let mut set: IndexSet<char> = ('a'..='z').collect();

    // Insert '*' at the beginning (index 0)
    assert_eq!(set.insert_before(0, '*'), (0, true));
    assert_eq!(set.get_index_of(&'*'), Some(0));
    assert_eq!(set.get_index_of(&'a'), Some(1));
}

