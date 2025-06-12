// Answer 0

#[test]
fn test_slice_new() {
    struct TestKey;
    struct TestValue;

    let slice = Slice::<TestKey, TestValue>::new();

    assert!(slice.is_empty());
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_slice_new_mut() {
    struct TestKey;
    struct TestValue;

    let slice_mut = Slice::<TestKey, TestValue>::new_mut();

    assert!(slice_mut.is_empty());
    assert_eq!(slice_mut.len(), 0);
}

