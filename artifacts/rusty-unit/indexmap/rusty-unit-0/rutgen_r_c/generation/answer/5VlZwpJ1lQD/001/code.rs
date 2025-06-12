// Answer 0

#[test]
fn test_len_with_empty_indices() {
    let index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    assert_eq!(index_map.len(), 0);
}

#[test]
fn test_len_with_non_empty_indices() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices.push(1); // Assuming the push method is available in Indices
    index_map.indices.push(2);
    assert_eq!(index_map.len(), 2);
}

#[test]
fn test_len_after_clear() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.indices.push(1);
    index_map.indices.clear(); // Assuming clear method is available in Indices
    assert_eq!(index_map.len(), 0);
}

