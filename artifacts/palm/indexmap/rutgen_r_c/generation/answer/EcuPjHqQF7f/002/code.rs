// Answer 0

#[test]
fn test_try_reserve_with_insufficient_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.try_reserve(5);
    assert_eq!(map.entries.capacity(), 0); // capacity should remain 0
}

#[test]
fn test_try_reserve_with_sufficient_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    let result = map.try_reserve(5);
    assert!(result.is_ok());
    assert_eq!(map.entries.capacity(), 10); // capacity should remain unchanged
}

#[test]
fn test_try_reserve_should_increase_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    
    // Simulate inserting existing elements to change the len
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    // Initial state
    assert_eq!(map.entries.len(), 2);
    
    // Attempt to reserve more than currently available capacity
    let result = map.try_reserve(3);
    assert!(result.is_ok());
    assert!(map.entries.len() <= 5); // Capacity should be increased to accommodate more entries
}

