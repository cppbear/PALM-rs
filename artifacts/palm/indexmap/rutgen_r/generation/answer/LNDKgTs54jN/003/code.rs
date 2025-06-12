// Answer 0

#[test]
fn test_shift_insert_existing_key_valid_move() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='e').map(|c| (c, ())).collect();

    // Move existing key 'a' from index 0 to 2
    assert_eq!(map.get_index_of(&'a'), Some(0));
    assert_eq!(map.shift_insert(2, 'a', ()), Some(()));
    assert_eq!(map.get_index_of(&'a'), Some(2));
    assert_eq!(map.get_index_of(&'b'), Some(0));
}

#[test]
fn test_shift_insert_existing_key_valid_move_multiple_shifts() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='e').map(|c| (c, ())).collect();

    // Move existing key 'b' from index 1 to 3
    assert_eq!(map.get_index_of(&'b'), Some(1));
    assert_eq!(map.shift_insert(3, 'b', ()), Some(()));
    assert_eq!(map.get_index_of(&'b'), Some(3));
    assert_eq!(map.get_index_of(&'c'), Some(2));
}

#[test]
#[should_panic]
fn test_shift_insert_existing_key_invalid_move() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='e').map(|c| (c, ())).collect();

    // Attempt to move key 'c' from valid index 2 to index 5 (out of bounds)
    assert_eq!(map.get_index_of(&'c'), Some(2));
    map.shift_insert(5, 'c', ());
}

#[test]
fn test_shift_insert_existing_key_last_index_move() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='e').map(|c| (c, ())).collect();

    // Move existing key 'e' from index 4 to 3
    assert_eq!(map.get_index_of(&'e'), Some(4));
    assert_eq!(map.shift_insert(3, 'e', ()), Some(()));
    assert_eq!(map.get_index_of(&'e'), Some(3));
}

