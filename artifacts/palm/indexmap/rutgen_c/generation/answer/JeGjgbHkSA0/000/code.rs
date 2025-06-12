// Answer 0

#[test]
fn test_into_keys_basic() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
    ];
    let slice: Box<Slice<_, _>> = Box::new(Slice { entries });
    let keys_iter = slice.into_keys().iter.collect::<Vec<_>>();
    assert_eq!(keys_iter, vec![1, 2]);
}

#[test]
fn test_into_keys_empty() {
    let slice: Box<Slice<i32, &str>> = Box::new(Slice { entries: [] });
    let keys_iter = slice.into_keys().iter.collect::<Vec<_>>();
    assert_eq!(keys_iter.len(), 0);
}

