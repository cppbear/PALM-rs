// Answer 0

#[test]
fn test_as_slice() {
    #[derive(Copy, Clone, Debug)]
    struct HashValue(u64);

    let bucket1 = Bucket {
        hash: HashValue(1),
        key: "key1",
        value: "value1",
    };
    let bucket2 = Bucket {
        hash: HashValue(2),
        key: "key2",
        value: "value2",
    };
    
    let entries = [bucket1, bucket2];
    let iter = Iter::new(&entries);
    
    let slice = iter.as_slice();
    
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, "key1");
    assert_eq!(slice.entries[1].key, "key2");
}

