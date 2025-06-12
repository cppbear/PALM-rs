// Answer 0

#[test]
fn test_split_first_mut_non_empty() {
    let mut bucket1 = Bucket { hash: 1, key: "key1", value: 10 };
    let mut bucket2 = Bucket { hash: 2, key: "key2", value: 20 };
    let mut buckets = [bucket1, bucket2];
    
    let mut slice = Slice::from_mut_slice(&mut buckets);
    
    if let Some((first, rest_slice)) = slice.split_first_mut() {
        assert_eq!(first.0, &"key1");
        assert_eq!(*first.1, 10);
        assert_eq!(rest_slice.entries.len(), 1);
        assert_eq!(rest_slice.entries[0].key, "key2");
    } else {
        panic!("Expected Some value but got None");
    }
}

#[test]
fn test_split_first_mut_empty() {
    let mut buckets: [Bucket<&str, i32>; 0] = [];
    let mut slice = Slice::from_mut_slice(&mut buckets);
    
    assert!(slice.split_first_mut().is_none());
}

