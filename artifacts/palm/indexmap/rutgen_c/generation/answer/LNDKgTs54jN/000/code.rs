// Answer 0

#[test]
fn test_shift_insert_new_entry() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_hasher(RandomState::new());
    assert_eq!(map.len(), 0);
    
    // Insert a new key '*' at index 0
    assert_eq!(map.shift_insert(0, '*', ()), None);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_index_of(&'*'), Some(0));
}

#[test]
fn test_shift_insert_existing_entry_move() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_hasher(RandomState::new());
    for c in 'a'..='d' {
        map.insert(c, ());
    }

    // Move the key 'a' to index 2
    assert_eq!(map.shift_insert(2, 'a', ()), Some(()));
    assert_eq!(map.get_index_of(&'a'), Some(2));
    assert_eq!(map.get_index_of(&'b'), Some(0));
    assert_eq!(map.get_index_of(&'c'), Some(1));
    assert_eq!(map.get_index_of(&'d'), Some(3));
}

#[test]
fn test_shift_insert_invalid_index_move() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_hasher(RandomState::new());
    for c in 'a'..='d' {
        map.insert(c, ());
    }

    // This is an invalid index for moving an existing key!
    let result = std::panic::catch_unwind(|| {
        map.shift_insert(map.len(), 'a', ());
    });
    
    assert!(result.is_err());
}

#[test]
fn test_shift_insert_at_end() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_hasher(RandomState::new());
    for c in 'a'..='d' {
        map.insert(c, ());
    }

    // Inserting a new entry at the end
    assert_eq!(map.shift_insert(map.len(), 'e', ()), None);
    assert_eq!(map.len(), 5);
    assert_eq!(map.get_index_of(&'e'), Some(4));
}

#[should_panic]
#[test]
fn test_shift_insert_out_of_bounds_move() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_hasher(RandomState::new());
    for c in 'a'..='d' {
        map.insert(c, ());
    }

    // This will panic due to index being out of bounds
    map.shift_insert(4, 'a', ());
}

