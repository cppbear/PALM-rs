// Answer 0

#[test]
fn test_split_at_empty_slice() {
    let slice = Slice::new();
    let (first, second) = slice.split_at(0);
    assert_eq!(first.len(), 0);
    assert_eq!(second.len(), 0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_at_panic_out_of_bounds() {
    let slice = Slice::new();
    let _ = slice.split_at(1);
}

#[test]
fn test_split_at_non_empty_slice() {
    let slice = Slice::from_slice(&[
        Bucket { hash: HashValue::default(), key: 1, value: 'a' },
        Bucket { hash: HashValue::default(), key: 2, value: 'b' },
        Bucket { hash: HashValue::default(), key: 3, value: 'c' },
    ]);
    let (first, second) = slice.split_at(2);
    assert_eq!(first.len(), 2);
    assert_eq!(second.len(), 1);
    assert_eq!(first.get_index(0).unwrap().key, 1);
    assert_eq!(first.get_index(1).unwrap().key, 2);
    assert_eq!(second.get_index(0).unwrap().key, 3);
}

#[test]
fn test_split_at_full_slice() {
    let slice = Slice::from_slice(&[
        Bucket { hash: HashValue::default(), key: 1, value: 'a' },
        Bucket { hash: HashValue::default(), key: 2, value: 'b' },
        Bucket { hash: HashValue::default(), key: 3, value: 'c' },
    ]);
    let (first, second) = slice.split_at(3);
    assert_eq!(first.len(), 3);
    assert_eq!(second.len(), 0);
}

#[test]
fn test_split_at_mid_slice() {
    let slice = Slice::from_slice(&[
        Bucket { hash: HashValue::default(), key: 1, value: 'a' },
        Bucket { hash: HashValue::default(), key: 2, value: 'b' },
        Bucket { hash: HashValue::default(), key: 3, value: 'c' },
        Bucket { hash: HashValue::default(), key: 4, value: 'd' },
    ]);
    
    let (first, second) = slice.split_at(1);
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 3);
    assert_eq!(first.get_index(0).unwrap().key, 1);
}

