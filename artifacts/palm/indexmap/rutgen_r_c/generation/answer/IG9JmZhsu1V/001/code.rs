// Answer 0

#[test]
fn test_len_empty_slice() {
    let slice: &Slice<i32> = Slice::new();
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_len_non_empty_slice() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "one" },
        Bucket { hash: HashValue::default(), key: 2, value: "two" },
    ];
    let slice = Box::new(Slice { entries });
    assert_eq!(slice.len(), 2);
}

#[test]
fn test_len_single_element_slice() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 1, value: "one" }];
    let slice = Box::new(Slice { entries });
    assert_eq!(slice.len(), 1);
}

