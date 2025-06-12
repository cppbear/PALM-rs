// Answer 0

#[test]
fn test_into_entries_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_into_entries_single_bucket() {
    let bucket = Bucket { hash: 0, key: 1, value: 2 };
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [bucket] });
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, 2);
}

#[test]
fn test_into_entries_multiple_buckets() {
    let bucket1 = Bucket { hash: 0, key: 1, value: 2 };
    let bucket2 = Bucket { hash: 1, key: 3, value: 4 };
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [bucket1, bucket2] });
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, 2);
    assert_eq!(entries[1].key, 3);
    assert_eq!(entries[1].value, 4);
}

