// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "a");
    map.insert(1, "b");
    map.insert(2, "c");

    let result = map.shift_remove_index(1);
    assert_eq!(result, Some("b"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&0), Some(&"a"));
    assert_eq!(map.get(&2), Some(&"c"));
}

#[test]
fn test_shift_remove_index_first() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "a");
    map.insert(1, "b");
    map.insert(2, "c");

    let result = map.shift_remove_index(0);
    assert_eq!(result, Some("a"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&"b"));
    assert_eq!(map.get(&2), Some(&"c"));
}

#[test]
fn test_shift_remove_index_last() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "a");
    map.insert(1, "b");
    map.insert(2, "c");

    let result = map.shift_remove_index(2);
    assert_eq!(result, Some("c"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&0), Some(&"a"));
    assert_eq!(map.get(&1), Some(&"b"));
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds_negative() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "a");
    
    let _result = map.shift_remove_index(usize::MAX); // Out of bounds
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds_too_large() {
    let mut map = indexmap::IndexMap::new();
    map.insert(0, "a");
    
    let _result = map.shift_remove_index(1); // Out of bounds
}

#[test]
fn test_shift_remove_index_empty() {
    let mut map = indexmap::IndexMap::new();
    let result = map.shift_remove_index(0);
    assert_eq!(result, None);
}

