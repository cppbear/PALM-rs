// Answer 0

#[test]
fn test_with_entries_empty() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.with_entries(|entries| {
        assert!(entries.is_empty());
    });
}

#[test]
fn test_with_entries_single() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.with_entries(|entries| {
        entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
        assert_eq!(entries.len(), 1);
    });
}

#[test]
fn test_with_entries_multiple() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.with_entries(|entries| {
        entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
        entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 });
        assert_eq!(entries.len(), 2);
    });
}

#[test]
fn test_with_entries_large_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(100);
    index_map.with_entries(|entries| {
        for i in 0..100 {
            entries.push(Bucket { hash: HashValue::default(), key: i, value: i * 10 });
        }
        assert_eq!(entries.len(), 100);
    });
}

#[test]
#[should_panic]
fn test_with_entries_exceeding_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(2);
    index_map.with_entries(|entries| {
        entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
        entries.push(Bucket { hash: HashValue::default(), key: 2, value: 2 });
        entries.push(Bucket { hash: HashValue::default(), key: 3, value: 3 }); // This should panic
    });
}

#[test]
fn test_with_entries_retaining_order() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.with_entries(|entries| {
        entries.push(Bucket { hash: HashValue::default(), key: 4, value: 40 });
        entries.push(Bucket { hash: HashValue::default(), key: 5, value: 50 });
    });
    index_map.with_entries(|entries| {
        assert_eq!(entries[0].key, 4);
        assert_eq!(entries[1].key, 5);
    });
}

