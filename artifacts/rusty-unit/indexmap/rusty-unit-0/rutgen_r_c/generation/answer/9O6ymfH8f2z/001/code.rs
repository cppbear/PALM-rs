// Answer 0

#[test]
fn test_keys_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let keys = slice.keys();
    assert_eq!(keys.iter.len(), 0);
}

#[test]
fn test_keys_non_empty_slice() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];

    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let keys = slice.keys();
    let collected_keys: Vec<_> = keys.iter.map(|bucket| &bucket.key).collect();
    
    assert_eq!(collected_keys, vec![&1, &2, &3]);
}

#[test]
fn test_keys_single_element_slice() {
    let entries = [Bucket { hash: 0, key: 1, value: 10 }];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let keys = slice.keys();
    let collected_keys: Vec<_> = keys.iter.map(|bucket| &bucket.key).collect();
    
    assert_eq!(collected_keys, vec![&1]);
}

