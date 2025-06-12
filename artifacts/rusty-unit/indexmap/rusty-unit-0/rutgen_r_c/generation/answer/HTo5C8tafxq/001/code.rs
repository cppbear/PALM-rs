// Answer 0

#[test]
fn test_with_entries_empty() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
    });
}

#[test]
fn test_with_entries_non_empty() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(3);
    index_map.entries.push(Bucket { hash: 0.into(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: 1.into(), key: 2, value: 20 });

    index_map.with_entries(|entries| {
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].key, 1);
        assert_eq!(entries[0].value, 10);
        assert_eq!(entries[1].key, 2);
        assert_eq!(entries[1].value, 20);
    });
}

#[test]
fn test_with_entries_modify_entries() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    
    index_map.with_entries(|entries| {
        entries.push(Bucket { hash: 0.into(), key: 1, value: 10 });
        entries.push(Bucket { hash: 1.into(), key: 2, value: 20 });
    });
    
    assert_eq!(index_map.entries.len(), 2);
    assert_eq!(index_map.entries[0].key, 1);
    assert_eq!(index_map.entries[0].value, 10);
    assert_eq!(index_map.entries[1].key, 2);
    assert_eq!(index_map.entries[1].value, 20);
}

#[test]
fn test_with_entries_rebuild_hash_table() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    let mut modified_entries = Vec::new();
    
    index_map.with_entries(|entries| {
        modified_entries.push(Bucket { hash: 0.into(), key: 3, value: 30 });
        modified_entries.push(Bucket { hash: 1.into(), key: 4, value: 40 });
        
        entries.clear();
        entries.extend(modified_entries.iter().cloned());
    });

    assert_eq!(index_map.entries.len(), 2);
    assert_eq!(index_map.entries[0].key, 3);
    assert_eq!(index_map.entries[0].value, 30);
    assert_eq!(index_map.entries[1].key, 4);
    assert_eq!(index_map.entries[1].value, 40);
}

