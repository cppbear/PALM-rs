// Answer 0

#[test]
fn test_shrink_to_below_current_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    
    // Shrink to a value below the current capacity
    map.shrink_to(10);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_shrink_to_zero() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    
    // Shrink to zero
    map.shrink_to(0);
    assert!(map.capacity() >= 2); // should not drop below the minimum capacity
}

#[test]
fn test_shrink_to_no_op() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    assert!(map.capacity() >= 100);
    
    // Attempt to shrink to a value that is already less than current capacity
    map.shrink_to(200);
    assert!(map.capacity() >= 100); // no operation; capacity remains unchanged
}

#[test]
fn test_shrink_to_current_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(50);
    assert!(map.capacity() >= 50);
}

#[test]
fn test_shrink_to_exact_capacity() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity(80);
    for i in 0..75 {
        map.insert(i, i);
    }
    
    // Shrink to exactly 75
    map.shrink_to(75);
    assert!(map.capacity() >= 75);
}

