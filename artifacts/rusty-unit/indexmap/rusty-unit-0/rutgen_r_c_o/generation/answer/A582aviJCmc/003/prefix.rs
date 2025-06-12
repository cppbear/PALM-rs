// Answer 0

#[test]
fn test_insert_before_with_key_moving_to_occupied_index() {
    let mut map: IndexMap<char, usize> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    let old_value = map.insert('c', 3);
    
    let (index, old) = map.insert_before(1, 'a', 5);
    assert!(index == 0);
    assert!(old.is_some() && old.unwrap() == 1);
}

#[test]
fn test_insert_before_with_key_at_the_same_index() {
    let mut map: IndexMap<char, usize> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.insert('x', 10);
    map.insert('y', 20);
    
    let (index, old) = map.insert_before(1, 'x', 15);
    assert!(index == 1);
    assert!(old.is_some() && old.unwrap() == 10);
}

#[test]
fn test_insert_before_panic_condition_out_of_bounds() {
    let mut map: IndexMap<char, usize> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.insert('m', 30);
    
    let result = std::panic::catch_unwind(|| {
        map.insert_before(2, 'n', 35);
    });
    assert!(result.is_err());
}

#[test]
fn test_insert_before_with_key_moving_up() {
    let mut map: IndexMap<char, usize> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.insert('d', 4);
    map.insert('e', 5);
    map.insert('f', 6);
    
    let (index, old) = map.insert_before(0, 'd', 8);
    assert!(index == 0);
    assert!(old.is_some() && old.unwrap() == 4);
}

#[test]
fn test_insert_before_with_key_remains_same() {
    let mut map: IndexMap<char, usize> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.insert('g', 7);
    
    let (index, old) = map.insert_before(0, 'g', 9);
    assert!(index == 0);
    assert!(old.is_some() && old.unwrap() == 7);
}

