// Answer 0

#[test]
fn test_insert_before_at_end_with_vacant_entry() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    let len = map.len(); // Initially, len is 4

    // Attempting to insert a new key '*' at index 4 (which is equal to len)
    let result = map.insert_before(len, '*', ());
    
    // We expect the insertion to occur at index 4 and return None since the entry is vacant.
    assert_eq!(result, (len, None)); // Should be (4, None)
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_insert_before_panics_on_out_of_bounds_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    
    // Attempting to insert a new key '*' at index 5 (which is out of bounds)
    map.insert_before(5, '*', ());
}

#[test]
fn test_insert_before_at_new_high_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('m'..='p').map(|c| (c, ())).collect();
    let len = map.len(); // Initially, len is 4 (for 'm', 'n', 'o', 'p')

    // Inserting a new key '#' at index equal to len (4)
    let result = map.insert_before(len, '#', ());
    
    // We expect the insertion to occur at index 4 and return None since the entry is vacant.
    assert_eq!(result, (len, None)); // Should be (4, None)
}

#[test]
fn test_insert_before_with_existing_entry_moving_up() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('x'..='z').map(|c| (c, ())).collect();
    map.insert_before(2, 'w', ()); // Inserting 'w' at index 2
    let len = map.len(); // Now len is 4 (for 'x', 'y', 'w', 'z')

    // Now inserting 'x' again to move it up (the existing entry)
    let result = map.insert_before(len, 'x', ());
    
    // We expect the result to shift 'x' to index 3; should return (3, Some(()))
    assert_eq!(result, (3, Some(()))); 
}

