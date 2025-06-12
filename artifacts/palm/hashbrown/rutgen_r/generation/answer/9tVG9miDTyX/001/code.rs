// Answer 0

#[test]
fn test_shrink_to_valid_min_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    
    // Ensure initial capacity is at least 100
    assert!(set.capacity() >= 100);
    
    set.shrink_to(10);
    
    // Ensure capacity is at least 10 after shrinking
    assert!(set.capacity() >= 10);
    
    set.shrink_to(0);
    
    // Ensure capacity is not less than the number of elements
    assert!(set.capacity() >= 2);
}

#[test]
#[should_panic]
fn test_shrink_to_panic_below_current_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(5);
    set.insert(1);
    set.insert(2);
    
    // Attempting to shrink below the current capacity should panic
    set.shrink_to(3);
    set.shrink_to(1); // This is valid, will not panic

    // This should panic, as current capacity is above min capacity.
    set.shrink_to(0); // Should not panic, as it allows up to 2 items.
}

#[test]
fn test_shrink_to_exact_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(20);
    set.insert(1);
    set.insert(2);
    
    // Check initial capacity.
    assert!(set.capacity() >= 20);
    
    set.shrink_to(2);
    
    // Ensure capacity remains the same as the number of elements.
    assert_eq!(set.capacity(), 2);
}

#[test]
fn test_shrink_to_minimum() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(50);
    set.insert(10);
    
    // Ensure initial capacity
    assert!(set.capacity() >= 50);
    
    // Shrink to a capacity of 1
    set.shrink_to(1);
    
    // Ensure capacity is still valid and within bounds
    assert!(set.capacity() >= 1);
}

#[test]
#[should_panic]
fn test_shrink_to_panic_on_empty_set() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(10);
    
    // Empty set, should panic when trying to shrink to a non-zero capacity
    set.shrink_to(1);
}

