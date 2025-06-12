// Answer 0

#[test]
fn test_from_boxed_valid_case() {
    let bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    
    let boxed_array = Box::new([bucket1, bucket2]);
    let slice_boxed = Slice::from_boxed(boxed_array);
    
    assert_eq!(slice_boxed.entries.len(), 2);
    assert_eq!(slice_boxed.entries[0].key, "key1");
    assert_eq!(slice_boxed.entries[1].key, "key2");
}

#[test]
#[should_panic]
fn test_from_boxed_empty() {
    let boxed_array: Box<[Bucket<&str, &str>]> = Box::new([]);
    let _slice_boxed = Slice::from_boxed(boxed_array);
}

