// Answer 0

#[test]
fn test_truncate_less_than_current_length() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    assert_eq!(index_map.len(), 2);
    index_map.truncate(1);
    
    assert_eq!(index_map.len(), 1);
    assert_eq!(index_map.entries.len(), 1);
}

#[test]
fn test_truncate_equal_to_current_length() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    assert_eq!(index_map.len(), 2);
    index_map.truncate(2);
    
    assert_eq!(index_map.len(), 2);
    assert_eq!(index_map.entries.len(), 2);
}

#[test]
fn test_truncate_greater_than_current_length() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    
    assert_eq!(index_map.len(), 1);
    index_map.truncate(2);
    
    assert_eq!(index_map.len(), 1); // Length should not change
    assert_eq!(index_map.entries.len(), 1);
}

#[test]
fn test_truncate_to_zero() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    
    assert_eq!(index_map.len(), 1);
    index_map.truncate(0);
    
    assert_eq!(index_map.len(), 0);
    assert_eq!(index_map.entries.len(), 0);
}

