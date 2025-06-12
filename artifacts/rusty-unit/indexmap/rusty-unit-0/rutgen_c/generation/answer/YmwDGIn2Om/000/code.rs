// Answer 0

#[test]
fn test_with_capacity_zero() {
    let index_map = IndexMapCore::<usize, usize>::with_capacity(0);
    assert_eq!(index_map.capacity(), 0);
    assert_eq!(index_map.len(), 0);
}

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let index_map = IndexMapCore::<usize, usize>::with_capacity(capacity);
    assert_eq!(index_map.capacity(), capacity);
    assert_eq!(index_map.len(), 0);
}

#[test]
fn test_with_capacity_large_value() {
    let capacity = IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY;
    let index_map = IndexMapCore::<usize, usize>::with_capacity(capacity);
    assert_eq!(index_map.capacity(), capacity);
    assert_eq!(index_map.len(), 0);
}

