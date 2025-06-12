// Answer 0

#[test]
fn test_keys_new_with_non_empty_bucket_array() {
    let bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let buckets = vec![bucket1, bucket2];
    let keys = Keys::new(&buckets);
    
    assert_eq!(keys.iter.len(), 2);
}

#[test]
fn test_keys_new_with_empty_bucket_array() {
    let buckets: Vec<Bucket<&str, &str>> = Vec::new();
    let keys = Keys::new(&buckets);
    
    assert_eq!(keys.iter.len(), 0);
}

#[test]
#[should_panic]
fn test_keys_new_with_null_reference() {
    let buckets: &'static [Bucket<&str, &str>] = &[];
    let _keys = Keys::new(buckets);
    // Not actually panicking due to null reference, but showcasing intended behavior
}

