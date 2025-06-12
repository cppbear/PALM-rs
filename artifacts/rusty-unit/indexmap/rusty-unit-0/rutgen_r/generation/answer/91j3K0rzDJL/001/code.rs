// Answer 0

#[test]
fn test_shift_insert_within_bounds() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let (key_ref, value_ref) = map.shift_insert(1, 3, 30);
    assert_eq!(*key_ref, 3);
    assert_eq!(*value_ref, 30);
    assert_eq!(map.len(), 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_insert_out_of_bounds_high() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    let _ = map.shift_insert(3, 3, 30); // Out of bounds
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_insert_out_of_bounds_low() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    let _ = map.shift_insert(usize::MAX, 3, 30); // Out of bounds
}

#[test]
fn test_shift_insert_at_start() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let (key_ref, value_ref) = map.shift_insert(0, 3, 30);
    assert_eq!(*key_ref, 3);
    assert_eq!(*value_ref, 30);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&3), Some(&30));
}

#[test]
fn test_shift_insert_at_end() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let (key_ref, value_ref) = map.shift_insert(2, 3, 30);
    assert_eq!(*key_ref, 3);
    assert_eq!(*value_ref, 30);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&3), Some(&30));
}

