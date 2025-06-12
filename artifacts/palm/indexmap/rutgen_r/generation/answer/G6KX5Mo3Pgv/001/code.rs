// Answer 0

#[test]
fn test_truncate_with_zero_length() {
    let mut map = indexmap::IndexMap::<i32, i32>::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    map.truncate(0);
    
    // After truncating to 0, the map should be empty
    assert_eq!(map.len(), 0);
}

#[test]
fn test_truncate_with_less_than_current_length() {
    let mut map = indexmap::IndexMap::<i32, i32>::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    map.truncate(2);
    
    // After truncating to 2, map should have the first 2 elements
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&100));
    assert_eq!(map.get(&2), Some(&200));
}

#[test]
fn test_truncate_with_equal_length() {
    let mut map = indexmap::IndexMap::<i32, i32>::new();
    map.insert(1, 100);
    map.insert(2, 200);
    
    map.truncate(2);
    
    // After truncating to the current length of 2, the map should remain the same
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&100));
    assert_eq!(map.get(&2), Some(&200));
}

#[test]
fn test_truncate_with_greater_than_current_length() {
    let mut map = indexmap::IndexMap::<i32, i32>::new();
    map.insert(1, 100);
    map.insert(2, 200);
    
    map.truncate(5);
    
    // After truncating to a greater length, the map should remain the same
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&100));
    assert_eq!(map.get(&2), Some(&200));
}

#[test]
fn test_truncate_empty_map() {
    let mut map = indexmap::IndexMap::<i32, i32>::new();
    
    map.truncate(2); // Truncating an empty map should have no effect
    
    // Map should still be empty
    assert_eq!(map.len(), 0);
}

