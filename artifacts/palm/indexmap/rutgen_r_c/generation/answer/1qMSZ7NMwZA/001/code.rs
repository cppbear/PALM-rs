// Answer 0

#[test]
fn test_len_empty_slice() {
    let slice: Slice<u32, u32> = Slice { entries: [] };
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_len_non_empty_slice() {
    let entries = [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
    ];
    let slice: Slice<u32, u32> = Slice { entries };
    assert_eq!(slice.len(), 2);
}

#[test]
fn test_len_large_slice() {
    let entries = (0..1000)
        .map(|i| Bucket { hash: HashValue::default(), key: i, value: i * 10 })
        .collect::<Vec<_>>();
    let slice: Slice<u32, u32> = Slice { entries: entries.try_into().unwrap() };
    assert_eq!(slice.len(), 1000);
}

