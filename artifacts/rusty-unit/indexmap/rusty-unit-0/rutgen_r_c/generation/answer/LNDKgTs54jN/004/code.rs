// Answer 0

#[test]
fn test_shift_insert_existing_key_move_down() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // Move key 'a' down to index 10
    assert_eq!(map.shift_insert(10, 'a', ()), Some(()));
    assert_eq!(map.get_index_of(&'a'), Some(10));
}

#[test]
fn test_shift_insert_existing_key_move_up() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // Move key 'z' down to index 9
    assert_eq!(map.shift_insert(9, 'z', ()), Some(()));
    assert_eq!(map.get_index_of(&'z'), Some(9));
}

#[test]
fn test_shift_insert_new_key() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // Insert a new key '*' at the end
    assert_eq!(map.shift_insert(map.len(), '*', ()), None);
    assert_eq!(map.get_index_of(&'*'), Some(26));
}

#[test]
fn test_shift_insert_panic_move_existing_key_out_of_bounds() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // Expect panic when trying to move existing key 'a' to index equal to len
    let result = std::panic::catch_unwind(|| {
        map.shift_insert(map.len(), 'a', ());
    });
    assert!(result.is_err());
}

#[test]
fn test_shift_insert_existing_key_move_to_len_minus_one() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // Move key '*' to the last valid index (len - 1)
    assert_eq!(map.shift_insert(map.len() - 1, '*', ()), Some(()));
    assert_eq!(map.get_index_of(&'*'), Some(25));
}

