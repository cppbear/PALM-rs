// Answer 0

#[test]
fn test_keys_new() {
    // Given a sample slice of Bucket<K, V>
    let entries: Vec<Bucket<i32, &str>> = vec![
        Bucket { hash: HashValue::new(1), key: 1, value: "one" },
        Bucket { hash: HashValue::new(2), key: 2, value: "two" },
    ];

    // When creating a new Keys instance
    let keys_instance = Keys::new(&entries);

    // Then the iterator should be initialized correctly
    let mut iter = keys_instance.iter.clone();
    assert_eq!(iter.next().unwrap().key, 1);
    assert_eq!(iter.next().unwrap().key, 2);
    assert!(iter.next().is_none());
}

