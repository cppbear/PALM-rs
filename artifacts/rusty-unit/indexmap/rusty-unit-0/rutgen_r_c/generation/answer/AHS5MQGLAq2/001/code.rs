// Answer 0

#[test]
fn test_split_last_non_empty() {
    // Create a Slice with multiple entries
    let entry1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let entry2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let entry3 = Bucket { hash: 3, key: "key3", value: "value3" };
    
    let entries = [entry1, entry2, entry3];
    let slice: &Slice<_, _> = Slice::from_slice(&entries);

    // Call split_last and verify the result
    let result = slice.split_last();
    assert!(result.is_some());
    
    if let Some((last_pair, rest_slice)) = result {
        assert_eq!(last_pair, (&"key3", &"value3"));
        assert_eq!(rest_slice.len(), 2);
        assert_eq!(rest_slice.get_index(0), Some((&"key1", &"value1")));
        assert_eq!(rest_slice.get_index(1), Some((&"key2", &"value2")));
    } else {
        panic!("Expected Some result but got None");
    }
}

#[test]
fn test_split_last_empty() {
    // Create an empty Slice
    let entries: [Bucket<&str, &str>; 0] = [];
    let slice: &Slice<_, _> = Slice::from_slice(&entries);

    // Call split_last and verify the result
    let result = slice.split_last();
    assert!(result.is_none());
}

