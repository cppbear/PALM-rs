// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

#[test]
fn test_with_capacity_large_value() {
    let capacity = usize::MAX;
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

