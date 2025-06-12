// Answer 0

#[test]
fn test_from_boxed_empty() {
    let entries: Box<[Bucket<i32>]> = Box::new([]);
    let slice: Box<Slice<i32>> = Slice::from_boxed(entries);
    assert_eq!(slice.entries.len(), 0);
}

#[test]
fn test_from_boxed_single() {
    let bucket = Bucket { hash: 0, key: 1, value: "value" };
    let entries: Box<[Bucket<&str>]> = Box::new([bucket]);
    let slice: Box<Slice<&str>> = Slice::from_boxed(entries);
    assert_eq!(slice.entries.len(), 1);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, "value");
}

#[test]
fn test_from_boxed_multiple() {
    let bucket1 = Bucket { hash: 1, key: 1, value: "first" };
    let bucket2 = Bucket { hash: 2, key: 2, value: "second" };
    let entries: Box<[Bucket<&str>]> = Box::new([bucket1, bucket2]);
    let slice: Box<Slice<&str>> = Slice::from_boxed(entries);
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
}

#[should_panic]
fn test_from_boxed_invalid() {
    let entries: Box<[Bucket<i32>]> = Box::new([]);
    let slice: Box<Slice<i32>> = Slice::from_boxed(entries);
    // Trying to access an index on an empty slice should panic
    let _ = &slice[0];
}

