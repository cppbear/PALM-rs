// Answer 0

#[test]
fn test_into_entries_empty_slice() {
    let slice: Box<Slice<u32>> = Box::new(Slice::new());
    let entries = slice.into_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_into_entries_single_element() {
    let bucket = Bucket { hash: 0, key: 1, value: "one" };
    let slice: Box<Slice<&str>> = Box::new(Slice::from_slice(&[bucket]));
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, "one");
}

#[test]
fn test_into_entries_multiple_elements() {
    let buckets = [
        Bucket { hash: 0, key: 1, value: "one" },
        Bucket { hash: 1, key: 2, value: "two" },
        Bucket { hash: 2, key: 3, value: "three" },
    ];
    let slice: Box<Slice<&str>> = Box::new(Slice::from_slice(&buckets));
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].key, "one");
    assert_eq!(entries[1].key, "two");
    assert_eq!(entries[2].key, "three");
}

#[test]
fn test_into_entries_large_number_of_elements() {
    let buckets: Vec<Bucket<u32, &str>> = (0..1000)
        .map(|i| Bucket { hash: i, key: i, value: "value" })
        .collect();
    let slice: Box<Slice<&str>> = Box::new(Slice::from_slice(&buckets));
    let entries = slice.into_entries();
    assert_eq!(entries.len(), 1000);
    assert_eq!(entries[0].key, 0);
    assert_eq!(entries[999].key, 999);
}

