// Answer 0

#[test]
fn test_shrink_to() {
    use crate::hashbrown::HashMap;

    // Create a HashMap with an initial capacity
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(10, 20);
    map.insert(30, 40);
    
    // Assert initial capacity
    assert!(map.capacity() >= 100);
    
    // Shrink to a valid minimum capacity
    map.shrink_to(10);
    // Assert that the capacity is at least 10
    assert!(map.capacity() >= 10);
    
    // Shrink to a lower capacity than the current size of the map
    map.shrink_to(5);
    // Assert that the capacity is at least 10, since we can't go lower than current size
    assert!(map.capacity() >= 10);

    // Shrink to a very small capacity
    map.shrink_to(2);
    // Assert that the capacity is at least 2
    assert!(map.capacity() >= 2);
    
    // Try shrinking to zero capacity
    map.shrink_to(0);
    // Assert that the capacity is at least 2, since it's the minimum threshold
    assert!(map.capacity() >= 2);
}

#[test]
fn test_shrink_to_zero() {
    use crate::hashbrown::HashMap;

    // Create a HashMap and insert elements
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);
    map.insert(1, 2);
    map.insert(3, 4);
    
    // Assert initial capacity
    assert!(map.capacity() >= 50);
    
    // Shrink to zero
    map.shrink_to(0);
    // Assert that the capacity is not zero
    assert!(map.capacity() > 0);
} 

#[test]
fn test_shrink_to_smaller_capacity_than_elements() {
    use crate::hashbrown::HashMap;

    // Create a HashMap and insert multiple elements
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
    map.insert(1, 2);
    map.insert(2, 3);
    
    // Assert initial capacity
    assert!(map.capacity() >= 10);
    
    // Attempt to shrink below the number of elements in the map
    map.shrink_to(1);
    // Assert that the capacity is still at least as large as the number of elements
    assert!(map.capacity() >= 2);
} 

#[test]
fn test_shrink_to_no_operation() {
    use crate::hashbrown::HashMap;

    // Create a HashMap with a small capacity
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(5);
    map.insert(1, 2);
    
    // Assert initial capacity
    assert!(map.capacity() >= 5);
    
    // Shrink to a capacity larger than current
    map.shrink_to(7);
    // Assert that the capacity did not change (it should still be 5 or larger)
    assert!(map.capacity() >= 5);
}

