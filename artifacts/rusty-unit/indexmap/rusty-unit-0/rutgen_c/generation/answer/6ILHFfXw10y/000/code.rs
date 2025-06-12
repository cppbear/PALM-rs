// Answer 0

#[test]
fn test_as_slice_single_entry() {
    let bucket = Bucket { hash: HashValue::default(), key: 1, value: "value1" };
    let entries = vec![bucket];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), 1);
}

#[test]
fn test_as_slice_multiple_entries() {
    let bucket1 = Bucket { hash: HashValue::default(), key: 1, value: "value1" };
    let bucket2 = Bucket { hash: HashValue::default(), key: 2, value: "value2" };
    let entries = vec![bucket1, bucket2];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), 2);
}

#[test]
fn test_as_slice_empty_iterator() {
    let entries: Vec<Bucket<i32, &str>> = vec![];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), 0);
}

