// Answer 0

#[test]
fn test_borrow_mut_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let ref_mut = index_map.borrow_mut();
    assert!(ref_mut.indices.is_empty());
    assert!(ref_mut.entries.is_empty());
}

#[test]
fn test_borrow_mut_after_insertion() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash_value = HashValue::default(); // assuming a default HashValue exists
    index_map.push_entry(hash_value, 1, 100);
    let ref_mut = index_map.borrow_mut();
    assert_eq!(ref_mut.entries.len(), 1);
}

#[test]
fn test_borrow_mut_multiple_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash_value1 = HashValue::default();
    let hash_value2 = HashValue::default(); // would generally be different in a real scenario
    index_map.push_entry(hash_value1, 1, 100);
    index_map.push_entry(hash_value2, 2, 200);
    let ref_mut = index_map.borrow_mut();
    assert_eq!(ref_mut.entries.len(), 2);
}

#[test]
fn test_borrow_mut_on_large_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1000);
    for i in 0..1000 {
        let hash_value = HashValue::default();
        index_map.push_entry(hash_value, i, i as usize);
    }
    let ref_mut = index_map.borrow_mut();
    assert_eq!(ref_mut.entries.len(), 1000);
}

#[test]
fn test_borrow_mut_with_truncation() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..10 {
        let hash_value = HashValue::default();
        index_map.push_entry(hash_value, i, i as usize);
    }
    index_map.truncate(5);
    let ref_mut = index_map.borrow_mut();
    assert_eq!(ref_mut.entries.len(), 5);
}

