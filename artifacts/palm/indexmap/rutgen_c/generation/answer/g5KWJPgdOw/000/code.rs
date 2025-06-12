// Answer 0

#[test]
fn test_is_empty_with_empty_slice() {
    let slice: &Slice<i32, i32> = Slice::new();
    assert!(slice.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_slice() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice = Slice { entries };
    assert!(!slice.is_empty());
}

