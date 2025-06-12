// Answer 0

#[test]
fn test_capacity_with_initial_capacity() {
    use hashbrown::HashMap;
    
    let map: HashMap<i32, i32> = HashMap::with_capacity(100);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 100);
}

#[test]
fn test_capacity_after_insertions() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);
    assert!(map.capacity() >= 50);
    
    map.insert(1, 1);
    map.insert(2, 2);
    
    assert_eq!(map.len(), 2);
    assert!(map.capacity() >= 50); // Ensure capacity does not decrease
}

#[test]
fn test_capacity_with_no_elements() {
    use hashbrown::HashMap;
    
    let map: HashMap<i32, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    assert!(map.capacity() > 0); // Capacity must be greater than zero for a new map
}

#[test]
fn test_capacity_zero_initialization() {
    use hashbrown::HashMap;
    
    let map: HashMap<i32, i32> = HashMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() == 0); // Check that capacity is exactly zero
}

