// Answer 0

#[test]
fn test_insert_before_new_key() {
    let mut map = crate::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    assert_eq!(map.len(), 0);
    let (index, old_value) = map.insert_before(0, 'a', 1);
    assert_eq!(index, 0);
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_index_of(&'a'), Some(0));
}

#[test]
fn test_insert_before_move_existing_key() {
    let mut map = crate::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    let (index, old_value) = map.insert_before(1, 'a', 3);
    assert_eq!(index, 0);
    assert_eq!(old_value, Some(1));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get_index_of(&'a'), Some(0));
    assert_eq!(map.get_index_of(&'b'), Some(1));
}

#[test]
fn test_insert_before_at_end() {
    let mut map = crate::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    let (index, old_value) = map.insert_before(2, 'c', 3);
    assert_eq!(index, 2);
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get_index_of(&'c'), Some(2));
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_insert_before_out_of_bounds() {
    let mut map = crate::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert_before(3, 'c', 3); // index 3 is out of bounds
}

#[test]
fn test_insert_before_shift_existing_key_up() {
    let mut map = crate::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    let (index, old_value) = map.insert_before(1, 'b', 4);
    assert_eq!(index, 0);
    assert_eq!(old_value, Some(2));
    assert_eq!(map.get_index_of(&'b'), Some(0));
    assert_eq!(map.get_index_of(&'a'), Some(1));
    assert_eq!(map.get_index_of(&'c'), Some(2));
}

