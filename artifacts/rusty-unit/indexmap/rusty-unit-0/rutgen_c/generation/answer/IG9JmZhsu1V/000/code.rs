// Answer 0

#[test]
fn test_len_empty_slice() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_len_non_empty_slice() {
    let entries = [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
    ];
    let slice = Slice { entries };
    assert_eq!(slice.len(), 2);
}

#[test]
fn test_len_full_slice() {
    let entries: [Bucket<i32, &str>; 3] = [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let slice = Slice { entries };
    assert_eq!(slice.len(), 3);
}

