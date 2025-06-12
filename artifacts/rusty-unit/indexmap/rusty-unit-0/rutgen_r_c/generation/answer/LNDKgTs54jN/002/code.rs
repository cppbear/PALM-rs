// Answer 0

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds_for_existing_key() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    
    // Attempting to move an existing key 'a' to an out-of-bounds index (map.len()).
    map.shift_insert(map.len(), 'a', ());
}

#[test]
fn test_shift_insert_insert_new_key_at_end() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    
    // The new key '*' goes exactly at the given index which is equal to the map length.
    assert_eq!(map.shift_insert(map.len(), '*', ()), None);
    assert_eq!(map.len(), 5);
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds_for_new_key() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    
    // Attempting to insert a new key '*' at an out-of-bounds index (map.len() + 1).
    map.shift_insert(map.len() + 1, '*', ());
}

#[test]
fn test_shift_insert_existing_key_move_down() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    
    // Moving the key 'c' down to index 1 will shift others up.
    assert_eq!(map.shift_insert(1, 'c', ()), Some(()));
    assert_eq!(map.len(), 4);
}

#[test]
fn test_shift_insert_existing_key_move_up() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    
    // Moving the key 'a' up to index 2 will shift 'b' down.
    assert_eq!(map.shift_insert(2, 'a', ()), Some(()));
    assert_eq!(map.len(), 4);
}

