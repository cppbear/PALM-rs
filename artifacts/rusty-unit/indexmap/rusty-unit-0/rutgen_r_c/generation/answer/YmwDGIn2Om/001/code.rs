// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_small_number() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_with_capacity_large_number() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

