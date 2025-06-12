// Answer 0

#[test]
fn test_shift_insert_new_key_at_valid_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_of(&'*'), None);
    assert_eq!(map.shift_insert(10, '*', ()), None);
    assert_eq!(map.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_existing_key_move_down() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.shift_insert(10, 'a', ()), Some(()));
    assert_eq!(map.get_index_of(&'a'), Some(10));
}

#[test]
fn test_shift_insert_existing_key_move_up() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.shift_insert(9, 'z', ()), Some(()));
    assert_eq!(map.get_index_of(&'z'), Some(9));
}

#[test]
fn test_shift_insert_existing_key_to_len_minus_one() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.len(), 27);
    assert_eq!(map.shift_insert(map.len() - 1, '*', ()), Some(()));
    assert_eq!(map.get_index_of(&'*'), Some(26));
}

#[test]
fn test_shift_insert_new_key_at_len() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.shift_insert(map.len(), '+', ()), None);
    assert_eq!(map.get_index_of(&'+'), Some(27));
    assert_eq!(map.len(), 28);
}

#[should_panic]
fn test_shift_insert_move_existing_key_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(map.len(), 'a', ());
}

