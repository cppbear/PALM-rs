// Answer 0

#[test]
fn test_values_mut_new() {
    // Create a mutable vector of Bucket<K, V>
    let mut buckets: Vec<Bucket<i32, String>> = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
        Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() },
    ];

    // Initialize ValuesMut with the mutable slice of buckets
    let values_mut = ValuesMut::new(&mut buckets);

    // Verify that the iterator is functioning
    let mut iter = values_mut.iter;
    assert_eq!(iter.len(), 3); // Ensure iterator sees 3 buckets

    // Now test that the iterator returns the buckets as expected
    let first = iter.next().unwrap();
    assert_eq!(first.key, 1);
    assert_eq!(first.value, "one");

    let second = iter.next().unwrap();
    assert_eq!(second.key, 2);
    assert_eq!(second.value, "two");

    let third = iter.next().unwrap();
    assert_eq!(third.key, 3);
    assert_eq!(third.value, "three");

    assert!(iter.next().is_none()); // Ensure no more items
}

