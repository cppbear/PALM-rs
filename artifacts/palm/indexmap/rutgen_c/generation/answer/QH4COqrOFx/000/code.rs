// Answer 0

#[test]
fn test_last_empty_slice() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.last(), None);
}

#[test]
fn test_last_single_element_slice() {
    let bucket = Bucket { hash: 0, key: 42, value: 0 };
    let entries = [bucket];
    let slice = Slice { entries };
    assert_eq!(slice.last(), Some(&42));
}

#[test]
fn test_last_multiple_elements_slice() {
    let bucket1 = Bucket { hash: 1, key: 15, value: 0 };
    let bucket2 = Bucket { hash: 2, key: 30, value: 0 };
    let bucket3 = Bucket { hash: 3, key: 42, value: 0 };
    let entries = [bucket1, bucket2, bucket3];
    let slice = Slice { entries };
    assert_eq!(slice.last(), Some(&42));
}

#[test]
fn test_last_consistency() {
    let bucket1 = Bucket { hash: 1, key: 10, value: 0 };
    let bucket2 = Bucket { hash: 2, key: 20, value: 0 };
    let bucket3 = Bucket { hash: 3, key: 30, value: 0 };
    let entries = [bucket1, bucket2, bucket3];
    let slice = Slice { entries };
    assert_eq!(slice.last(), Some(&30));
    assert!(!slice.is_empty());
}

