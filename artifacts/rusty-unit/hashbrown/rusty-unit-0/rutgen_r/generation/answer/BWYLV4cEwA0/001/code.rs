// Answer 0

#[test]
fn test_capacity_with_initial_capacity() {
    use hashbrown::HashMap;

    let map: HashMap<i32, i32> = HashMap::with_capacity(100);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 100);
}

#[test]
fn test_capacity_with_default_initialization() {
    use hashbrown::HashMap;

    let map: HashMap<i32, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    assert!(map.capacity() > 0); // Default capacity should be greater than 0
}

#[test]
fn test_capacity_after_inserting_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
    for i in 0..10 {
        map.insert(i, i);
    }
    assert_eq!(map.len(), 10);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_capacity_with_large_number() {
    use hashbrown::HashMap;

    let map: HashMap<i32, i32> = HashMap::with_capacity(1_000_000);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 1_000_000);
}

#[test]
fn test_capacity_zero_initial_capacity() {
    use hashbrown::HashMap;

    let map: HashMap<i32, i32> = HashMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() == 0); // Should be exactly 0 for zero initial capacity
}

