// Answer 0

#[test]
fn test_reverse_with_indices_non_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let bucket1 = Bucket { hash: HashValue::default(), key: 1, value: 100 };
    let bucket2 = Bucket { hash: HashValue::default(), key: 2, value: 200 };

    // Simulating a scenario where indices and entries are populated
    index_map.entries.push(bucket1);
    index_map.entries.push(bucket2);
    index_map.indices.push(1);
    index_map.indices.push(0);
    
    index_map.reverse();
    
    assert_eq!(index_map.entries.len(), 2);
    assert_eq!(index_map.entries[0].key, 2);
    assert_eq!(index_map.entries[1].key, 1);
    assert_eq!(index_map.indices[0], 0);
    assert_eq!(index_map.indices[1], 1);
}

#[test]
fn test_reverse_with_indices_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();

    // Call reverse on empty IndexMapCore
    index_map.reverse();
    
    // Assert that the state remains unchanged for an empty structure
    assert_eq!(index_map.entries.len(), 0);
    assert_eq!(index_map.indices.len(), 0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_reverse_on_invalid_state() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 100 });

    // Manually setting an invalid index to simulate panic scenario
    index_map.indices.push(1);  // This index does not correspond to any valid entry
    index_map.reverse();
}

