// Answer 0

#[test]
fn test_shrink_to_smaller_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    // Initial capacity check
    assert!(map.capacity() >= 3);
    
    map.shrink_to(2);
    
    // Check if the capacity is reduced but still greater than or equal to 2
    assert!(map.capacity() >= 2);
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    // Initial capacity check
    assert!(map.capacity() >= 2);
    
    map.shrink_to(2);
    
    // Check if the capacity is exactly 2
    assert_eq!(map.capacity(), 2);
}

#[test]
fn test_shrink_to_greater_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    
    // Initial capacity check
    assert!(map.capacity() >= 1);
    
    map.shrink_to(2);
    
    // Check if the capacity is still at least 2
    assert!(map.capacity() >= 2);
}

#[test]
fn test_shrink_to_zero_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    
    // Map is empty, capacity should be 0
    assert_eq!(map.capacity(), 0);
    
    map.shrink_to(0);
    
    // Capacity should remain 0
    assert_eq!(map.capacity(), 0);
}

#[test]
#[should_panic]
fn test_shrink_to_negative_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.shrink_to(usize::MAX); // This should panic or cause an error, assuming it doesn't support large capacities
}

