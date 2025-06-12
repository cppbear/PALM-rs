// Answer 0

#[test]
fn test_insert_full_occupied_entry() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let hash1 = HashValue(1);
    index_map.insert_full(hash1, 10, 20);
    
    let hash2 = HashValue(1);
    let result = index_map.insert_full(hash2, 10, 30);
}

#[test]
fn test_insert_full_multiple_occupied_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let hash1 = HashValue(1);
    index_map.insert_full(hash1, 10, 20);
    
    let hash2 = HashValue(2);
    index_map.insert_full(hash2, 20, 40);
    
    let hash3 = HashValue(1);
    let result = index_map.insert_full(hash3, 10, 30);
}

#[test]
fn test_insert_full_with_same_hash_different_keys() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let hash1 = HashValue(1);
    index_map.insert_full(hash1, 10, 20);
    
    let hash2 = HashValue(1);
    let result = index_map.insert_full(hash2, 30, 40);
}

#[test]
fn test_insert_full_with_empty_map() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let hash1 = HashValue(1);
    let result = index_map.insert_full(hash1, 10, 20);
}

#[test]
fn test_insert_full_replaces_existing_value() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let hash1 = HashValue(1);
    index_map.insert_full(hash1, 10, 20);
    
    let result = index_map.insert_full(hash1, 10, 30);
}

