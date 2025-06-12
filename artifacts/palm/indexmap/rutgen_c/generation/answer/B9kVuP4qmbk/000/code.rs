// Answer 0

#[test]
fn test_into_entries_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let entries = slice.into_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_into_entries_single_element() {
    let bucket = Bucket { hash: 0, key: 1, value: 100 };
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [bucket] });
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, 100);
}

#[test]
fn test_into_entries_multiple_elements() {
    let buckets = [
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 2, value: 200 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: buckets });
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, 100);
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[1].value, 200);
}

