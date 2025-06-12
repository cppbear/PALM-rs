// Answer 0

#[test]
fn test_split_first_non_empty() {
    struct TestBucket {
        key: usize,
        value: usize,
    }
    
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
        Bucket { hash: 0, key: 3, value: 30 },
    ];
    
    let slice = Slice::from_slice(&entries);
    
    let (first_key, rest) = slice.split_first().unwrap();
    
    assert_eq!(*first_key, 1);
    assert_eq!(rest.len(), 2);
}

#[test]
fn test_split_first_empty() {
    let entries: [Bucket<usize>; 0] = [];
    let slice = Slice::from_slice(&entries);
    
    assert!(slice.split_first().is_none());
}

