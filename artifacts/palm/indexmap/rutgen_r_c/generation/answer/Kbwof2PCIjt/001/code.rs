// Answer 0

#[test]
fn test_as_slice_with_non_empty_iterator() {
    let entries = vec![
        Bucket { hash: 1, key: 'a', value: 10 },
        Bucket { hash: 2, key: 'b', value: 20 },
    ];
    let mut iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 'a');
    assert_eq!(slice.entries[0].value, 10);
    assert_eq!(slice.entries[1].key, 'b');
    assert_eq!(slice.entries[1].value, 20);
}

#[test]
fn test_as_slice_with_empty_iterator() {
    let entries: Vec<Bucket<char, i32>> = vec![];
    let mut iter = IntoIter::new(entries);
    let slice = iter.as_slice();

    assert_eq!(slice.entries.len(), 0);
}

