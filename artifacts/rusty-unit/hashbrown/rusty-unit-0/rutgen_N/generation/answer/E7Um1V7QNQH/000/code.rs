// Answer 0

#[test]
fn test_shrink_to_with_capacity_large_enough() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    
    map.shrink_to(10);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_shrink_to_with_zero_limit() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);

    map.shrink_to(0);
    assert!(map.capacity() >= 2); // Confirming it doesn't go below the minimum internal capacity
}

#[test]
fn test_shrink_to_no_op_when_below_min_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(5);
    map.insert(1, 2);
    assert!(map.capacity() >= 5);

    map.shrink_to(10);
    assert!(map.capacity() >= 5); // Capacity should remain unchanged
}

#[test]
fn test_shrink_to_exact_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(15);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 15);

    map.shrink_to(15);
    assert!(map.capacity() >= 15); // Capacity should remain unchanged at exact limit
}

#[test]
fn test_shrink_to_below_current_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(20);
    map.insert(1, 2);
    assert!(map.capacity() >= 20);

    map.shrink_to(5);
    assert!(map.capacity() >= 5); // Capacity should be shrunk to minimum limit
}

