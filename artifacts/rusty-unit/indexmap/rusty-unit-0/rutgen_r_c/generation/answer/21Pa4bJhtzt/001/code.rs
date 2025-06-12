// Answer 0

#[test]
fn test_reserve_growth_when_capacity_is_insufficient() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(5);
    
    // Initially, the length is 0, so capacity is 5.
    // We attempt to reserve more than the remaining capacity
    let additional = 6;
    map.reserve(additional);

    // Now we check the length after reserving
    assert!(map.len() >= additional);
}

#[test]
fn test_reserve_exact_growth_check() {
    let mut map = IndexMapCore::<usize, usize>::with_capacity(10);
    
    // Initially, the length is 0, capacity is 10
    let additional = 11;
    
    // This should not panic
    map.reserve(additional);
    
    // Check that the map can now hold more than its initial capacity
    assert!(map.len() >= additional);
}

