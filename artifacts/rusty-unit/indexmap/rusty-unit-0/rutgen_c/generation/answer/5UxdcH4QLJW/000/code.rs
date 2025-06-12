// Answer 0

#[test]
fn test_into_entries_empty_slice() {
    let slice: Box<Slice<u32>> = Box::new(Slice::new());
    let entries = slice.into_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_into_entries_single_element() {
    let bucket = Bucket { hash: 0, key: 1, value: "value" };
    let slice: Box<Slice<&str>> = Box::new(Slice::from_slice(&[bucket]));
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "value");
}

#[test]
fn test_into_entries_multiple_elements() {
    let buckets = [
        Bucket { hash: 0, key: 1, value: "value1" },
        Bucket { hash: 1, key: 2, value: "value2" },
    ];
    let slice: Box<Slice<&str>> = Box::new(Slice::from_slice(&buckets));
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "value1");
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[1].value, "value2");
}

