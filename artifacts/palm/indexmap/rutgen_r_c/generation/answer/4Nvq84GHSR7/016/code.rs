// Answer 0

#[test]
fn test_erase_indices_no_erased() {
    let mut index_map: IndexMapCore<u32, String> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "one".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "two".to_string() });
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);
    
    // This should not panic as there are no erased entries.
    index_map.erase_indices(0, 0);
    assert_eq!(index_map.indices.len(), 2);
}

#[test]
fn test_erase_indices_half_capacity() {
    let mut index_map: IndexMapCore<u32, String> = IndexMapCore::with_capacity(4);
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "one".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "two".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "three".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 4, value: "four".to_string() });
    
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);
    index_map.indices.insert(3, 2);
    index_map.indices.insert(4, 3);
    
    // Constraint: start + shifted == half_capacity
    index_map.erase_indices(2, 4);
    assert_eq!(index_map.indices.len(), 2);
    assert_eq!(index_map.indices[0], 0);
    assert_eq!(index_map.indices[1], 1);
}

#[test]
fn test_erase_indices_erased_and_shifted() {
    let mut index_map: IndexMapCore<u32, String> = IndexMapCore::with_capacity(5);
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "one".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "two".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "three".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 4, value: "four".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(5), key: 5, value: "five".to_string() });
    
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);
    index_map.indices.insert(3, 2);
    index_map.indices.insert(4, 3);
    index_map.indices.insert(5, 4);
    
    // Constraint: erased + shifted < half_capacity
    index_map.erase_indices(1, 4);
    assert_eq!(index_map.indices.len(), 2);
    assert_eq!(index_map.indices[0], 0);
    assert_eq!(index_map.indices[1], 1);
}

#[test]
fn test_erase_indices_check_assertion() {
    let mut index_map: IndexMapCore<u32, String> = IndexMapCore::with_capacity(5);
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "one".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "two".to_string() });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "three".to_string() });
    
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);
    index_map.indices.insert(3, 2);
    
    // Panic condition: start + shifted < half_capacity is false, with bound start + shifted == half_capacity
    std::panic::catch_unwind(|| {
        index_map.erase_indices(0, 3);
    }).unwrap_err();
}

