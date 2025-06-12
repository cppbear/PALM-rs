// Answer 0

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let map: HashMap<&str, i32> = HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    // Assuming a capacity method exists for RawTable to check capacity
    assert!(map.table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_zero() {
    let capacity = 0;
    let map: HashMap<&str, i32> = HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    // Check that the capacity is exactly 0
    assert_eq!(map.table.capacity(), 0);
}

