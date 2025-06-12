// Answer 0

#[test]
fn test_into_boxed_empty() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [] });
    let result = slice.into_boxed();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_into_boxed_single_entry() {
    let bucket = Bucket { hash: HashValue(1), key: 1, value: "One" };
    let slice: Box<Slice<&str>> = Box::new(Slice { entries: [bucket] });
    let result = slice.into_boxed();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, 1);
    assert_eq!(result[0].value, "One");
}

#[test]
fn test_into_boxed_multiple_entries() {
    let buckets = [
        Bucket { hash: HashValue(1), key: 1, value: "One" },
        Bucket { hash: HashValue(2), key: 2, value: "Two" },
        Bucket { hash: HashValue(3), key: 3, value: "Three" },
    ];
    let slice: Box<Slice<&str>> = Box::new(Slice { entries: buckets });
    let result = slice.into_boxed();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].key, 1);
    assert_eq!(result[1].key, 2);
    assert_eq!(result[2].key, 3);
}

#[should_panic]
fn test_into_boxed_invalid_pointer() {
    // Create an invalid Box<Slice<T>> to trigger a panic.
    let invalid_slice: Box<Slice<i32>> = unsafe { Box::from_raw(std::ptr::null_mut()) };
    let _result = invalid_slice.into_boxed();
}

