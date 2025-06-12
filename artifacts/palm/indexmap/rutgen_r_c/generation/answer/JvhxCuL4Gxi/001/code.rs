// Answer 0

#[test]
fn test_reserve_exact_zero_additional() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve_exact(0);
    assert_eq!(index_map.len(), 0);
    assert_eq!(index_map.capacity(), 0);
}

#[test]
fn test_reserve_exact_non_zero_additional() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    index_map.reserve_exact(3);
    assert_eq!(index_map.len(), 0);
    assert!(index_map.capacity() >= 3);
}

#[test]
#[should_panic]
fn test_reserve_exact_over_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    index_map.reserve_exact(1);
}

