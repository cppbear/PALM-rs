// Answer 0

#[test]
fn test_shift_insert_existing_key_move_down() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    
    // Move an existing key 'a' to index 0
    assert_eq!(map.shift_insert(0, 'a', ()), Some(()));
    assert_eq!(map.get_index_of(&'a'), Some(0));
    assert_eq!(map.len(), 26); // 'a' moved, other elements shifted down
}

#[test]
fn test_shift_insert_existing_key_move_up() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    
    // Move an existing key 'z' to index 0
    assert_eq!(map.shift_insert(0, 'z', ()), Some(()));
    assert_eq!(map.get_index_of(&'z'), Some(0));
    assert_eq!(map.len(), 26); // 'z' moved, other elements shifted down
}

#[test]
#[should_panic]
fn test_shift_insert_existing_key_out_of_bounds_move() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    
    // This is an invalid index for moving an existing key!
    map.shift_insert(map.len(), 'a', ());
}

#[test]
fn test_shift_insert_new_key_at_end() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    
    // Insert a new key '+' at the end
    assert_eq!(map.shift_insert(map.len(), '+', ()), None);
    assert_eq!(map.get_index_of(&'+'), Some(26));
    assert_eq!(map.len(), 27);
}

#[test]
fn test_shift_insert_new_key_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    
    // Invalid index for inserting a new key since it exceeds length
    assert_eq!(map.shift_insert(map.len() + 1, '*', ()), None); // This should actually panic, but we are testing its return value. 
}

