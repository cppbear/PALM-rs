// Answer 0

#[test]
fn test_iter_mut_empty_slice() {
    let mut slice = Slice::new_mut();
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_single_entry() {
    let mut entries = vec![Bucket { hash: 0, key: "key1", value: "value1" }];
    let mut slice = Slice { entries: entries.try_into().unwrap() };
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
        Bucket { hash: 1, key: "key2", value: "value2" },
        Bucket { hash: 2, key: "key3", value: "value3" },
    ];
    let mut slice = Slice { entries: entries.try_into().unwrap() };
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_large_slice() {
    let entries: Vec<Bucket<usize, usize>> = (0..10000)
        .map(|i| Bucket { hash: i, key: i, value: i })
        .collect();
    let mut slice = Slice { entries: entries.try_into().unwrap() };
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_full_range() {
    let mut entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
        Bucket { hash: 1, key: "key2", value: "value2" },
        Bucket { hash: 2, key: "key3", value: "value3" },
        Bucket { hash: 3, key: "key4", value: "value4" },
    ];
    let mut slice = Slice { entries: entries.try_into().unwrap() };
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_boundaries() {
    let mut entries = vec![
        Bucket { hash: 0, key: "key1", value: "value1" },
        Bucket { hash: 1, key: "key2", value: "value2" },
    ];
    let mut slice = Slice { entries: entries.try_into().unwrap() };
    let mut iter = slice.iter_mut();
}

