// Answer 0

#[test]
fn test_with_entries_empty() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
    });
}

#[test]
fn test_with_entries_single() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: 10 });
    
    index_map.with_entries(|entries| {
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, 1);
        assert_eq!(entries[0].value, 10);
    });
}

#[test]
fn test_with_entries_multiple() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.entries.push(Bucket { hash: HashValue::from(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::from(2), key: 2, value: 20 });
    
    index_map.with_entries(|entries| {
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].key, 1);
        assert_eq!(entries[1].key, 2);
    });
}

