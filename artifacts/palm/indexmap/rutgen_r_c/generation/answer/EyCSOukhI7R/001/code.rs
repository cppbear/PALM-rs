// Answer 0

#[test]
fn test_as_mut_slice_non_empty() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
    ];
    let mut iter = IntoIter::new(buckets);
    let slice = iter.as_mut_slice();
    assert_eq!(slice.entries.len(), 2);
}

#[test]
fn test_as_mut_slice_single_element() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
    ];
    let mut iter = IntoIter::new(buckets);
    let slice = iter.as_mut_slice();
    assert_eq!(slice.entries.len(), 1);
}

#[test]
fn test_as_mut_slice_empty() {
    let buckets: Vec<Bucket<_, _>> = Vec::new();
    let mut iter = IntoIter::new(buckets);
    let slice = iter.as_mut_slice();
    assert_eq!(slice.entries.len(), 0);
}

#[should_panic]
fn test_as_mut_slice_panics_after_consumed() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
    ];
    let mut iter = IntoIter::new(buckets);
    let _slice_first = iter.as_mut_slice(); // First call
    let _slice_second = iter.as_mut_slice(); // Should panic if it tries to access after the first consumption
}

