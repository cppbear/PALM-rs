// Answer 0

#[test]
fn test_clear_empty_index_map_core() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.clear();
    assert_eq!(index_map.len(), 0);
    assert_eq!(index_map.capacity(), 0);
}

#[test]
fn test_clear_non_empty_index_map_core() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    index_map.insert_full(1.into(), 1, 10);
    index_map.insert_full(2.into(), 2, 20);
    assert_eq!(index_map.len(), 2);
    index_map.clear();
    assert_eq!(index_map.len(), 0);
    assert_eq!(index_map.capacity(), 5); // Capacity remains unchanged
}

#[test]
fn test_clear_after_operations() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.insert_full(1.into(), 1, 10);
    index_map.insert_full(2.into(), 2, 20);
    index_map.clear();
    assert_eq!(index_map.len(), 0);
    index_map.insert_full(3.into(), 3, 30);
    assert_eq!(index_map.len(), 1);
}

