// Answer 0

#[test]
fn test_push_entry_when_capacity_is_not_full() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash_value = HashValue(1);
    index_map.push_entry(hash_value, 1, 100);
    assert_eq!(index_map.entries.len(), 1);
}

#[test]
fn test_push_entry_increases_length() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    let initial_length = index_map.entries.len();
    
    index_map.push_entry(HashValue(1), 1, 100);
    index_map.push_entry(HashValue(2), 2, 200);
    
    assert_eq!(index_map.entries.len(), initial_length + 2);
}

#[test]
fn test_push_entry_with_existing_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    index_map.push_entry(HashValue(1), 1, 100);
    index_map.push_entry(HashValue(2), 2, 200);
    
    assert_eq!(index_map.entries.len(), 2);
    index_map.push_entry(HashValue(3), 3, 300);
    assert_eq!(index_map.entries.len(), 3);
}

#[test]
fn test_push_entry_does_not_panic_when_capacity_increases() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    
    index_map.push_entry(HashValue(1), 1, 100); // capacity = 1, length = 1
    index_map.push_entry(HashValue(2), 2, 200); // capacity gets reserved
    
    assert_eq!(index_map.entries.len(), 2);
}

#[test]
fn test_push_entry_stores_correct_data() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    let hash_value_1 = HashValue(1);
    let hash_value_2 = HashValue(2);
    
    index_map.push_entry(hash_value_1, 1, 100);
    index_map.push_entry(hash_value_2, 2, 200);
    
    assert_eq!(index_map.entries[0].key, 1);
    assert_eq!(index_map.entries[0].value, 100);
    assert_eq!(index_map.entries[1].key, 2);
    assert_eq!(index_map.entries[1].value, 200);
}

