// Answer 0

#[test]
fn test_slice_new_empty() {
    struct TestKey;
    struct TestValue;

    let empty_slice: &Slice<TestKey, TestValue> = Slice::new();

    assert_eq!(empty_slice.len(), 0);
    assert!(empty_slice.is_empty());
}

#[test]
fn test_slice_new_empty_non_generic() {
    let empty_slice: &Slice<i32, String> = Slice::new();

    assert_eq!(empty_slice.len(), 0);
    assert!(empty_slice.is_empty());
}

